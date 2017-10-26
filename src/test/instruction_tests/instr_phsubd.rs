use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn phsubd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBD, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 6, 206], OperandSize::Dword)
}

#[test]
fn phsubd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBD, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledIndexed(EDI, EDX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 6, 52, 151], OperandSize::Dword)
}

#[test]
fn phsubd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBD, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 6, 208], OperandSize::Qword)
}

#[test]
fn phsubd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBD, operand1: Some(Direct(MM1)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Four, 1726214037, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 6, 140, 178, 149, 239, 227, 102], OperandSize::Qword)
}

#[test]
fn phsubd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 6, 224], OperandSize::Dword)
}

#[test]
fn phsubd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(ESI, Eight, 1333924087, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 6, 20, 245, 247, 16, 130, 79], OperandSize::Dword)
}

#[test]
fn phsubd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 6, 192], OperandSize::Qword)
}

#[test]
fn phsubd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Two, 1024667062, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 6, 132, 80, 182, 45, 19, 61], OperandSize::Qword)
}

