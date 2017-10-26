use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmaxsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSW, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 238, 211], OperandSize::Dword)
}

#[test]
fn pmaxsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSW, operand1: Some(Direct(MM5)), operand2: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 238, 46], OperandSize::Dword)
}

#[test]
fn pmaxsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSW, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 238, 233], OperandSize::Qword)
}

#[test]
fn pmaxsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSW, operand1: Some(Direct(MM2)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Four, 1594179348, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 238, 148, 155, 20, 63, 5, 95], OperandSize::Qword)
}

#[test]
fn pmaxsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 238, 255], OperandSize::Dword)
}

#[test]
fn pmaxsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSW, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 238, 50], OperandSize::Dword)
}

#[test]
fn pmaxsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 238, 230], OperandSize::Qword)
}

#[test]
fn pmaxsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSW, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 1838393772, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 238, 4, 213, 172, 169, 147, 109], OperandSize::Qword)
}

