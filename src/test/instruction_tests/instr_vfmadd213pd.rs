use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmadd213pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 168, 209], OperandSize::Dword)
}

#[test]
fn vfmadd213pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 2124163975, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 168, 20, 69, 135, 43, 156, 126], OperandSize::Dword)
}

#[test]
fn vfmadd213pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 168, 254], OperandSize::Qword)
}

#[test]
fn vfmadd213pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(RDX, 434252026, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 168, 146, 250, 40, 226, 25], OperandSize::Qword)
}

#[test]
fn vfmadd213pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 245, 168, 211], OperandSize::Dword)
}

#[test]
fn vfmadd213pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(ECX, 2029236233, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 253, 168, 153, 9, 176, 243, 120], OperandSize::Dword)
}

#[test]
fn vfmadd213pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 221, 168, 218], OperandSize::Qword)
}

#[test]
fn vfmadd213pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 213, 168, 6], OperandSize::Qword)
}

#[test]
fn vfmadd213pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 229, 138, 168, 199], OperandSize::Dword)
}

#[test]
fn vfmadd213pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(EDX, EAX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 237, 141, 168, 12, 194], OperandSize::Dword)
}

#[test]
fn vfmadd213pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 221, 157, 168, 43], OperandSize::Dword)
}

#[test]
fn vfmadd213pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM21)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 213, 134, 168, 226], OperandSize::Qword)
}

#[test]
fn vfmadd213pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM9)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 1340409761, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 181, 141, 168, 28, 221, 161, 7, 229, 79], OperandSize::Qword)
}

#[test]
fn vfmadd213pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(RCX, RDX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 114, 253, 157, 168, 36, 209], OperandSize::Qword)
}

#[test]
fn vfmadd213pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 237, 169, 168, 220], OperandSize::Dword)
}

#[test]
fn vfmadd213pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 245, 175, 168, 58], OperandSize::Dword)
}

#[test]
fn vfmadd213pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(EBX, ECX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 213, 191, 168, 52, 139], OperandSize::Dword)
}

#[test]
fn vfmadd213pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM9)), operand3: Some(Direct(YMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 162, 181, 169, 168, 205], OperandSize::Qword)
}

#[test]
fn vfmadd213pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(RSI, 1683498692, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 229, 170, 168, 158, 196, 38, 88, 100], OperandSize::Qword)
}

#[test]
fn vfmadd213pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM25)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Eight, 1668743260, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 181, 178, 168, 180, 216, 92, 0, 119, 99], OperandSize::Qword)
}

#[test]
fn vfmadd213pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 197, 254, 168, 243], OperandSize::Dword)
}

#[test]
fn vfmadd213pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM2)), operand3: Some(Indirect(ESI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 237, 206, 168, 14], OperandSize::Dword)
}

#[test]
fn vfmadd213pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 2061133177, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 221, 222, 168, 60, 125, 121, 101, 218, 122], OperandSize::Dword)
}

#[test]
fn vfmadd213pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM20)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 221, 246, 168, 206], OperandSize::Qword)
}

#[test]
fn vfmadd213pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM12)), operand3: Some(IndirectDisplaced(RSI, 1758164548, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 157, 204, 168, 182, 68, 118, 203, 104], OperandSize::Qword)
}

#[test]
fn vfmadd213pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM31)), operand3: Some(IndirectDisplaced(RDI, 307597138, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 133, 211, 168, 143, 82, 143, 85, 18], OperandSize::Qword)
}

