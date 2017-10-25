use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmppd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(93)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 194, 210, 93], OperandSize::Dword)
}

#[test]
fn cmppd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPPD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(EBX, ESI, Four, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(97)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 194, 60, 179, 97], OperandSize::Dword)
}

#[test]
fn cmppd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(48)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 194, 219, 48], OperandSize::Qword)
}

#[test]
fn cmppd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPPD, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(108)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 194, 38, 108], OperandSize::Qword)
}

