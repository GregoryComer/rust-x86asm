use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsub213pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 170, 246], OperandSize::Dword)
}

#[test]
fn vfmsub213pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 1615982848, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 170, 60, 117, 0, 241, 81, 96], OperandSize::Dword)
}

#[test]
fn vfmsub213pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 170, 251], OperandSize::Qword)
}

#[test]
fn vfmsub213pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Two, 265584413, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 170, 156, 82, 29, 127, 212, 15], OperandSize::Qword)
}

#[test]
fn vfmsub213pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 245, 170, 211], OperandSize::Dword)
}

#[test]
fn vfmsub213pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Eight, 547888650, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 221, 170, 172, 255, 10, 30, 168, 32], OperandSize::Dword)
}

#[test]
fn vfmsub213pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 205, 170, 224], OperandSize::Qword)
}

#[test]
fn vfmsub213pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(RAX, 60852225, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 213, 170, 144, 1, 136, 160, 3], OperandSize::Qword)
}

#[test]
fn vfmsub213pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 213, 137, 170, 232], OperandSize::Dword)
}

#[test]
fn vfmsub213pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(ECX, EDX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 213, 137, 170, 12, 145], OperandSize::Dword)
}

#[test]
fn vfmsub213pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(EBX, 1340586330, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 229, 153, 170, 155, 90, 185, 231, 79], OperandSize::Dword)
}

#[test]
fn vfmsub213pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM11)), operand3: Some(Direct(XMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 162, 165, 139, 170, 195], OperandSize::Qword)
}

#[test]
fn vfmsub213pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM26)), operand3: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 173, 135, 170, 25], OperandSize::Qword)
}

#[test]
fn vfmsub213pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM31)), operand3: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 114, 133, 148, 170, 18], OperandSize::Qword)
}

#[test]
fn vfmsub213pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 174, 170, 204], OperandSize::Dword)
}

#[test]
fn vfmsub213pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(EDI, EDX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 245, 172, 170, 52, 87], OperandSize::Dword)
}

#[test]
fn vfmsub213pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(EDX, EDX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 205, 191, 170, 60, 82], OperandSize::Dword)
}

#[test]
fn vfmsub213pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM19)), operand3: Some(Direct(YMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 34, 229, 162, 170, 201], OperandSize::Qword)
}

#[test]
fn vfmsub213pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM19)), operand3: Some(IndirectScaledIndexed(RDI, RDX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 229, 162, 170, 20, 151], OperandSize::Qword)
}

#[test]
fn vfmsub213pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM20)), operand3: Some(IndirectScaledIndexed(RAX, RDX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 221, 178, 170, 4, 144], OperandSize::Qword)
}

#[test]
fn vfmsub213pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 229, 220, 170, 209], OperandSize::Dword)
}

#[test]
fn vfmsub213pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Four, 1991910793, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 237, 207, 170, 156, 179, 137, 37, 186, 118], OperandSize::Dword)
}

#[test]
fn vfmsub213pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 678034774, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 221, 219, 170, 60, 141, 86, 253, 105, 40], OperandSize::Dword)
}

#[test]
fn vfmsub213pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM14)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 141, 186, 170, 214], OperandSize::Qword)
}

#[test]
fn vfmsub213pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM10)), operand3: Some(IndirectDisplaced(RSI, 2028750516, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 173, 205, 170, 174, 180, 70, 236, 120], OperandSize::Qword)
}

#[test]
fn vfmsub213pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM27)), operand3: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 165, 215, 170, 16], OperandSize::Qword)
}

