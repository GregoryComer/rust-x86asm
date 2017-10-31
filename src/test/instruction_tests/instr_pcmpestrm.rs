use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pcmpestrm_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPESTRM, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(46)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 96, 237, 46], OperandSize::Dword)
}

#[test]
fn pcmpestrm_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPESTRM, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(EBX, EBX, Two, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(40)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 96, 44, 91, 40], OperandSize::Dword)
}

#[test]
fn pcmpestrm_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPESTRM, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 96, 222, 5], OperandSize::Qword)
}

#[test]
fn pcmpestrm_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPESTRM, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Four, 1071893734, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(43)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 96, 164, 134, 230, 204, 227, 63, 43], OperandSize::Qword)
}

