use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmulhw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHW, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 229, 224], OperandSize::Dword)
}

#[test]
fn pmulhw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHW, operand1: Some(Direct(MM1)), operand2: Some(IndirectScaledIndexed(EAX, ESI, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 229, 12, 176], OperandSize::Dword)
}

#[test]
fn pmulhw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHW, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 229, 249], OperandSize::Qword)
}

#[test]
fn pmulhw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHW, operand1: Some(Direct(MM2)), operand2: Some(IndirectDisplaced(RAX, 855628074, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 229, 144, 42, 217, 255, 50], OperandSize::Qword)
}

#[test]
fn pmulhw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 229, 236], OperandSize::Dword)
}

#[test]
fn pmulhw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHW, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Four, 577853288, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 229, 132, 155, 104, 87, 113, 34], OperandSize::Dword)
}

#[test]
fn pmulhw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 229, 200], OperandSize::Qword)
}

#[test]
fn pmulhw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Four, 787101900, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 229, 180, 131, 204, 56, 234, 46], OperandSize::Qword)
}

