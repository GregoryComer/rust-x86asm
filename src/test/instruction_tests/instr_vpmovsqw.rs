use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovsqw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 126, 138, 36, 213], OperandSize::Dword)
}

#[test]
fn vpmovsqw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQW, operand1: Some(IndirectScaledIndexedDisplaced(EAX, ECX, Four, 1983912856, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 36, 164, 136, 152, 27, 64, 118], OperandSize::Dword)
}

#[test]
fn vpmovsqw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQW, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 50, 126, 139, 36, 204], OperandSize::Qword)
}

#[test]
fn vpmovsqw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQW, operand1: Some(IndirectScaledIndexed(RCX, RSI, Two, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 8, 36, 4, 113], OperandSize::Qword)
}

#[test]
fn vpmovsqw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 126, 169, 36, 204], OperandSize::Dword)
}

#[test]
fn vpmovsqw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQW, operand1: Some(IndirectScaledIndexed(ESI, ECX, Four, Some(OperandSize::Qword), None)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 36, 36, 142], OperandSize::Dword)
}

#[test]
fn vpmovsqw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQW, operand1: Some(Direct(XMM19)), operand2: Some(Direct(YMM25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 34, 126, 175, 36, 203], OperandSize::Qword)
}

#[test]
fn vpmovsqw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQW, operand1: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand2: Some(Direct(YMM27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 126, 40, 36, 31], OperandSize::Qword)
}

#[test]
fn vpmovsqw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 126, 206, 36, 254], OperandSize::Dword)
}

#[test]
fn vpmovsqw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQW, operand1: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 36, 54], OperandSize::Dword)
}

#[test]
fn vpmovsqw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQW, operand1: Some(Direct(XMM25)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 146, 126, 206, 36, 209], OperandSize::Qword)
}

#[test]
fn vpmovsqw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQW, operand1: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 72, 36, 38], OperandSize::Qword)
}

