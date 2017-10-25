use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmullw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULLW, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 213, 209], OperandSize::Dword)
}

#[test]
fn pmullw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULLW, operand1: Some(Direct(MM4)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Four, 24244316, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 213, 164, 144, 92, 240, 113, 1], OperandSize::Dword)
}

#[test]
fn pmullw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULLW, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 213, 195], OperandSize::Qword)
}

#[test]
fn pmullw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULLW, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledDisplaced(RBX, Two, 1014401193, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 213, 52, 93, 169, 136, 118, 60], OperandSize::Qword)
}

#[test]
fn pmullw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULLW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 213, 240], OperandSize::Dword)
}

#[test]
fn pmullw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULLW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(EDI, EAX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 213, 52, 135], OperandSize::Dword)
}

#[test]
fn pmullw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULLW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 213, 229], OperandSize::Qword)
}

#[test]
fn pmullw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULLW, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(RDX, RDI, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 213, 36, 122], OperandSize::Qword)
}

