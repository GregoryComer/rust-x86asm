use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pcmpgtb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTB, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 100, 225], OperandSize::Dword)
}

#[test]
fn pcmpgtb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTB, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledIndexed(EBX, ECX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 100, 52, 75], OperandSize::Dword)
}

#[test]
fn pcmpgtb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTB, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 100, 247], OperandSize::Qword)
}

#[test]
fn pcmpgtb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTB, operand1: Some(Direct(MM3)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Four, 1419563659, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 100, 156, 139, 139, 210, 156, 84], OperandSize::Qword)
}

#[test]
fn pcmpgtb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 100, 207], OperandSize::Dword)
}

#[test]
fn pcmpgtb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTB, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(EAX, EDI, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 100, 4, 248], OperandSize::Dword)
}

#[test]
fn pcmpgtb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 100, 247], OperandSize::Qword)
}

#[test]
fn pcmpgtb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTB, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(RBX, RDI, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 100, 44, 123], OperandSize::Qword)
}

