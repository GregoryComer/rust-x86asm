use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmadd213pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 172, 246], OperandSize::Dword)
}

#[test]
fn vfnmadd213pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 172, 46], OperandSize::Dword)
}

#[test]
fn vfnmadd213pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 172, 204], OperandSize::Qword)
}

#[test]
fn vfnmadd213pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(RBX, RDI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 172, 28, 123], OperandSize::Qword)
}

#[test]
fn vfnmadd213pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 213, 172, 234], OperandSize::Dword)
}

#[test]
fn vfnmadd213pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(EDI, EDI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 197, 172, 60, 191], OperandSize::Dword)
}

#[test]
fn vfnmadd213pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 229, 172, 227], OperandSize::Qword)
}

#[test]
fn vfnmadd213pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 2121470160, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 253, 172, 44, 93, 208, 16, 115, 126], OperandSize::Qword)
}

#[test]
fn vfnmadd213pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 221, 137, 172, 236], OperandSize::Dword)
}

#[test]
fn vfnmadd213pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 213, 140, 172, 43], OperandSize::Dword)
}

#[test]
fn vfnmadd213pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(EBX, 510219912, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 229, 159, 172, 131, 136, 86, 105, 30], OperandSize::Dword)
}

#[test]
fn vfnmadd213pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM16)), operand3: Some(Direct(XMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 50, 253, 132, 172, 212], OperandSize::Qword)
}

#[test]
fn vfnmadd213pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 205, 139, 172, 19], OperandSize::Qword)
}

#[test]
fn vfnmadd213pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(RSI, 1234740961, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 98, 205, 159, 172, 150, 225, 166, 152, 73], OperandSize::Qword)
}

#[test]
fn vfnmadd213pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 213, 173, 172, 209], OperandSize::Dword)
}

#[test]
fn vfnmadd213pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Eight, 2087140308, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 197, 171, 172, 188, 195, 212, 59, 103, 124], OperandSize::Dword)
}

#[test]
fn vfnmadd213pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 588914236, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 197, 188, 172, 44, 181, 60, 30, 26, 35], OperandSize::Dword)
}

#[test]
fn vfnmadd213pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM24)), operand3: Some(Direct(YMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 146, 189, 162, 172, 211], OperandSize::Qword)
}

#[test]
fn vfnmadd213pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM28)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Four, 376985508, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 157, 165, 172, 180, 146, 164, 87, 120, 22], OperandSize::Qword)
}

#[test]
fn vfnmadd213pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 532281327, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 237, 185, 172, 52, 253, 239, 247, 185, 31], OperandSize::Qword)
}

#[test]
fn vfnmadd213pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 205, 153, 172, 222], OperandSize::Dword)
}

#[test]
fn vfnmadd213pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 1550336213, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 245, 207, 172, 44, 149, 213, 64, 104, 92], OperandSize::Dword)
}

#[test]
fn vfnmadd213pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM5)), operand3: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 213, 217, 172, 48], OperandSize::Dword)
}

#[test]
fn vfnmadd213pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM31)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 2, 237, 253, 172, 215], OperandSize::Qword)
}

#[test]
fn vfnmadd213pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM22)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 143581111, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 205, 199, 172, 44, 221, 183, 223, 142, 8], OperandSize::Qword)
}

#[test]
fn vfnmadd213pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(ZMM24)), operand2: Some(Direct(ZMM23)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Two, 92542652, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 197, 215, 172, 132, 81, 188, 22, 132, 5], OperandSize::Qword)
}

