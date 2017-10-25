use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmpps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(16)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 194, 244, 16], OperandSize::Dword)
}

#[test]
fn cmpps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPPS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(ESI, ECX, Four, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(8)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 194, 52, 142, 8], OperandSize::Dword)
}

#[test]
fn cmpps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(46)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 194, 213, 46], OperandSize::Qword)
}

#[test]
fn cmpps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPPS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(RSI, Four, 1143834269, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(97)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 194, 52, 181, 157, 134, 45, 68, 97], OperandSize::Qword)
}

