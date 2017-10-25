use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmulhrsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHRSW, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 11, 218], OperandSize::Dword)
}

#[test]
fn pmulhrsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHRSW, operand1: Some(Direct(MM1)), operand2: Some(IndirectScaledIndexed(EDI, ESI, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 11, 12, 247], OperandSize::Dword)
}

#[test]
fn pmulhrsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHRSW, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 11, 229], OperandSize::Qword)
}

#[test]
fn pmulhrsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHRSW, operand1: Some(Direct(MM7)), operand2: Some(IndirectDisplaced(RAX, 1149845081, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 11, 184, 89, 62, 137, 68], OperandSize::Qword)
}

#[test]
fn pmulhrsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHRSW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 11, 195], OperandSize::Dword)
}

#[test]
fn pmulhrsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHRSW, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Four, 801736609, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 11, 132, 159, 161, 135, 201, 47], OperandSize::Dword)
}

#[test]
fn pmulhrsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHRSW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 11, 243], OperandSize::Qword)
}

#[test]
fn pmulhrsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHRSW, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 11, 8], OperandSize::Qword)
}

