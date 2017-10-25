use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmadd213pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 172, 229], OperandSize::Dword)
}

#[test]
fn vfnmadd213pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 1909389387, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 172, 20, 125, 75, 248, 206, 113], OperandSize::Dword)
}

#[test]
fn vfnmadd213pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 172, 225], OperandSize::Qword)
}

#[test]
fn vfnmadd213pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 1117934345, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 172, 20, 117, 9, 83, 162, 66], OperandSize::Qword)
}

#[test]
fn vfnmadd213pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 237, 172, 228], OperandSize::Dword)
}

#[test]
fn vfnmadd213pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(EDX, EDI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 237, 172, 36, 122], OperandSize::Dword)
}

#[test]
fn vfnmadd213pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 237, 172, 212], OperandSize::Qword)
}

#[test]
fn vfnmadd213pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 1231179939, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 213, 172, 4, 117, 163, 80, 98, 73], OperandSize::Qword)
}

#[test]
fn vfnmadd213pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 229, 139, 172, 212], OperandSize::Dword)
}

#[test]
fn vfnmadd213pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(ESI, ESI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 142, 172, 12, 182], OperandSize::Dword)
}

#[test]
fn vfnmadd213pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 798015378, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 221, 159, 172, 12, 149, 146, 191, 144, 47], OperandSize::Dword)
}

#[test]
fn vfnmadd213pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM14)), operand3: Some(Direct(XMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 162, 141, 137, 172, 204], OperandSize::Qword)
}

#[test]
fn vfnmadd213pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM17)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Four, 1993247097, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 245, 131, 172, 156, 128, 121, 137, 206, 118], OperandSize::Qword)
}

#[test]
fn vfnmadd213pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM31)), operand3: Some(IndirectDisplaced(RCX, 917100958, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 133, 150, 172, 137, 158, 217, 169, 54], OperandSize::Qword)
}

#[test]
fn vfnmadd213pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 221, 172, 172, 216], OperandSize::Dword)
}

#[test]
fn vfnmadd213pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(EDX, 1005981329, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 245, 172, 172, 178, 145, 14, 246, 59], OperandSize::Dword)
}

#[test]
fn vfnmadd213pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 1776864608, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 229, 188, 172, 12, 77, 96, 205, 232, 105], OperandSize::Dword)
}

#[test]
fn vfnmadd213pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM18)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 237, 162, 172, 204], OperandSize::Qword)
}

#[test]
fn vfnmadd213pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM15)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 249218541, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 133, 173, 172, 60, 245, 237, 197, 218, 14], OperandSize::Qword)
}

#[test]
fn vfnmadd213pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 213, 186, 172, 56], OperandSize::Qword)
}

#[test]
fn vfnmadd213pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 205, 156, 172, 238], OperandSize::Dword)
}

#[test]
fn vfnmadd213pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 321099473, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 221, 207, 172, 4, 197, 209, 150, 35, 19], OperandSize::Dword)
}

#[test]
fn vfnmadd213pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectDisplaced(EAX, 1222610360, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 237, 220, 172, 184, 184, 141, 223, 72], OperandSize::Dword)
}

#[test]
fn vfnmadd213pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 237, 218, 172, 239], OperandSize::Qword)
}

#[test]
fn vfnmadd213pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectDisplaced(RSI, 1886815173, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 229, 203, 172, 190, 197, 131, 118, 112], OperandSize::Qword)
}

#[test]
fn vfnmadd213pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM0)), operand3: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 217, 172, 43], OperandSize::Qword)
}

