use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pblendw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PBLENDW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 14, 215, 3], OperandSize::Dword)
}

#[test]
fn pblendw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PBLENDW, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(48)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 14, 46, 48], OperandSize::Dword)
}

#[test]
fn pblendw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PBLENDW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(36)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 14, 245, 36], OperandSize::Qword)
}

#[test]
fn pblendw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PBLENDW, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Four, 1290898472, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(120)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 14, 188, 131, 40, 140, 241, 76, 120], OperandSize::Qword)
}

