use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pcmpeqw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQW, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 117, 245], OperandSize::Dword)
}

#[test]
fn pcmpeqw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQW, operand1: Some(Direct(MM1)), operand2: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 117, 14], OperandSize::Dword)
}

#[test]
fn pcmpeqw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQW, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 117, 217], OperandSize::Qword)
}

#[test]
fn pcmpeqw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQW, operand1: Some(Direct(MM4)), operand2: Some(IndirectScaledDisplaced(RBX, Four, 1023317445, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 117, 36, 157, 197, 149, 254, 60], OperandSize::Qword)
}

#[test]
fn pcmpeqw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 117, 253], OperandSize::Dword)
}

#[test]
fn pcmpeqw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQW, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Four, 1852626775, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 117, 148, 187, 87, 215, 108, 110], OperandSize::Dword)
}

#[test]
fn pcmpeqw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 117, 219], OperandSize::Qword)
}

#[test]
fn pcmpeqw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQW, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(RBX, 1026610309, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 117, 171, 133, 212, 48, 61], OperandSize::Qword)
}

