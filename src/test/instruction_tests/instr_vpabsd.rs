use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpabsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 30, 221], OperandSize::Dword)
}

#[test]
fn vpabsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Two, 1362795442, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 30, 140, 119, 178, 155, 58, 81], OperandSize::Dword)
}

#[test]
fn vpabsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 30, 246], OperandSize::Qword)
}

#[test]
fn vpabsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(RSI, 1999161573, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 30, 174, 229, 200, 40, 119], OperandSize::Qword)
}

#[test]
fn vpabsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 30, 241], OperandSize::Dword)
}

#[test]
fn vpabsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSD, operand1: Some(Direct(YMM3)), operand2: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 30, 25], OperandSize::Dword)
}

#[test]
fn vpabsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 30, 232], OperandSize::Qword)
}

#[test]
fn vpabsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSD, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Two, 1743823137, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 30, 172, 123, 33, 161, 240, 103], OperandSize::Qword)
}

