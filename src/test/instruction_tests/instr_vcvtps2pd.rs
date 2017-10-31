use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtps2pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 90, 239], OperandSize::Dword)
}

#[test]
fn vcvtps2pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(EDX, Eight, 507782732, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 90, 44, 213, 76, 38, 68, 30], OperandSize::Dword)
}

#[test]
fn vcvtps2pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 90, 202], OperandSize::Qword)
}

#[test]
fn vcvtps2pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(RSI, 367279035, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 90, 134, 187, 59, 228, 21], OperandSize::Qword)
}

#[test]
fn vcvtps2pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 90, 250], OperandSize::Dword)
}

#[test]
fn vcvtps2pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledDisplaced(EBX, Two, 426884598, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 90, 28, 93, 246, 189, 113, 25], OperandSize::Dword)
}

#[test]
fn vcvtps2pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 90, 216], OperandSize::Qword)
}

#[test]
fn vcvtps2pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(YMM3)), operand2: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 90, 31], OperandSize::Qword)
}

#[test]
fn vcvtps2pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 124, 137, 90, 221], OperandSize::Dword)
}

#[test]
fn vcvtps2pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(EDI, Eight, 1740986266, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 124, 143, 90, 4, 253, 154, 87, 197, 103], OperandSize::Dword)
}

#[test]
fn vcvtps2pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 124, 139, 90, 243], OperandSize::Qword)
}

#[test]
fn vcvtps2pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(XMM20)), operand2: Some(IndirectScaledIndexed(RAX, RSI, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 124, 137, 90, 36, 112], OperandSize::Qword)
}

#[test]
fn vcvtps2pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 124, 171, 90, 251], OperandSize::Dword)
}

#[test]
fn vcvtps2pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexed(ECX, ECX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 124, 171, 90, 4, 201], OperandSize::Dword)
}

#[test]
fn vcvtps2pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(XMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 209, 124, 169, 90, 198], OperandSize::Qword)
}

#[test]
fn vcvtps2pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(YMM20)), operand2: Some(IndirectDisplaced(RDI, 54500824, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 124, 171, 90, 167, 216, 157, 63, 3], OperandSize::Qword)
}

#[test]
fn vcvtps2pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 124, 158, 90, 218], OperandSize::Dword)
}

#[test]
fn vcvtps2pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectDisplaced(EAX, 1015088528, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 124, 202, 90, 184, 144, 5, 129, 60], OperandSize::Dword)
}

#[test]
fn vcvtps2pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(YMM26)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 145, 124, 158, 90, 210], OperandSize::Qword)
}

#[test]
fn vcvtps2pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(ZMM9)), operand2: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 124, 206, 90, 11], OperandSize::Qword)
}

