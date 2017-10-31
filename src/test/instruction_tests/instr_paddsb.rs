use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn paddsb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSB, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 236, 193], OperandSize::Dword)
}

#[test]
fn paddsb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSB, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledIndexed(EDI, EDX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 236, 60, 87], OperandSize::Dword)
}

#[test]
fn paddsb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSB, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 236, 250], OperandSize::Qword)
}

#[test]
fn paddsb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSB, operand1: Some(Direct(MM2)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Two, 615874411, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 236, 148, 75, 107, 127, 181, 36], OperandSize::Qword)
}

#[test]
fn paddsb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 236, 218], OperandSize::Dword)
}

#[test]
fn paddsb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSB, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 1324151178, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 236, 28, 197, 138, 241, 236, 78], OperandSize::Dword)
}

#[test]
fn paddsb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 236, 228], OperandSize::Qword)
}

#[test]
fn paddsb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSB, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(RDI, Two, 834909943, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 236, 52, 125, 247, 182, 195, 49], OperandSize::Qword)
}

