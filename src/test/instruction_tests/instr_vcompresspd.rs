use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcompresspd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 138, 138, 216], OperandSize::Dword)
}

#[test]
fn vcompresspd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPD, operand1: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 253, 8, 138, 59], OperandSize::Dword)
}

#[test]
fn vcompresspd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPD, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 130, 253, 140, 138, 241], OperandSize::Qword)
}

#[test]
fn vcompresspd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPD, operand1: Some(IndirectDisplaced(RSI, 1777916352, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 253, 8, 138, 190, 192, 217, 248, 105], OperandSize::Qword)
}

#[test]
fn vcompresspd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 171, 138, 255], OperandSize::Dword)
}

#[test]
fn vcompresspd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPD, operand1: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 253, 40, 138, 40], OperandSize::Dword)
}

#[test]
fn vcompresspd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 253, 174, 138, 192], OperandSize::Qword)
}

#[test]
fn vcompresspd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPD, operand1: Some(IndirectDisplaced(RBX, 758518931, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 253, 40, 138, 155, 147, 20, 54, 45], OperandSize::Qword)
}

#[test]
fn vcompresspd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 207, 138, 233], OperandSize::Dword)
}

#[test]
fn vcompresspd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPD, operand1: Some(Indirect(EDI, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 253, 72, 138, 47], OperandSize::Dword)
}

#[test]
fn vcompresspd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPD, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 66, 253, 206, 138, 218], OperandSize::Qword)
}

#[test]
fn vcompresspd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPD, operand1: Some(IndirectScaledIndexed(RDX, RBX, Four, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 253, 72, 138, 4, 154], OperandSize::Qword)
}

