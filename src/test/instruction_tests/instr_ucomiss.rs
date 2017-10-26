use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn ucomiss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::UCOMISS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 46, 240], OperandSize::Dword)
}

#[test]
fn ucomiss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::UCOMISS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Four, 1771038472, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 46, 180, 179, 8, 231, 143, 105], OperandSize::Dword)
}

#[test]
fn ucomiss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::UCOMISS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 46, 246], OperandSize::Qword)
}

#[test]
fn ucomiss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::UCOMISS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(RCX, Two, 1188006103, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 46, 36, 77, 215, 136, 207, 70], OperandSize::Qword)
}

