use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsub213pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 170, 213], OperandSize::Dword)
}

#[test]
fn vfmsub213pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 170, 57], OperandSize::Dword)
}

#[test]
fn vfmsub213pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 170, 200], OperandSize::Qword)
}

#[test]
fn vfmsub213pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 1624122658, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 170, 52, 141, 34, 37, 206, 96], OperandSize::Qword)
}

#[test]
fn vfmsub213pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 237, 170, 249], OperandSize::Dword)
}

#[test]
fn vfmsub213pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(ESI, ECX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 253, 170, 36, 206], OperandSize::Dword)
}

#[test]
fn vfmsub213pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 229, 170, 206], OperandSize::Qword)
}

#[test]
fn vfmsub213pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Eight, 1516249670, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 213, 170, 172, 254, 70, 34, 96, 90], OperandSize::Qword)
}

#[test]
fn vfmsub213pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 213, 138, 170, 238], OperandSize::Dword)
}

#[test]
fn vfmsub213pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(ESI, 815642099, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 205, 138, 170, 142, 243, 181, 157, 48], OperandSize::Dword)
}

#[test]
fn vfmsub213pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(ESI, EDI, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 213, 156, 170, 12, 126], OperandSize::Dword)
}

#[test]
fn vfmsub213pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM25)), operand3: Some(Direct(XMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 162, 181, 132, 170, 211], OperandSize::Qword)
}

#[test]
fn vfmsub213pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 2077824889, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 245, 140, 170, 36, 205, 121, 23, 217, 123], OperandSize::Qword)
}

#[test]
fn vfmsub213pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM19)), operand3: Some(IndirectDisplaced(RSI, 1058662344, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 229, 150, 170, 158, 200, 231, 25, 63], OperandSize::Qword)
}

#[test]
fn vfmsub213pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 205, 172, 170, 222], OperandSize::Dword)
}

#[test]
fn vfmsub213pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Two, 1090479413, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 237, 173, 170, 188, 95, 53, 101, 255, 64], OperandSize::Dword)
}

#[test]
fn vfmsub213pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 245, 188, 170, 0], OperandSize::Dword)
}

#[test]
fn vfmsub213pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM31)), operand3: Some(Direct(YMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 162, 133, 166, 170, 200], OperandSize::Qword)
}

#[test]
fn vfmsub213pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM24)), operand3: Some(IndirectDisplaced(RDX, 1147001839, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 189, 165, 170, 178, 239, 219, 93, 68], OperandSize::Qword)
}

#[test]
fn vfmsub213pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM18)), operand3: Some(IndirectDisplaced(RDX, 1873920352, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 237, 183, 170, 170, 96, 193, 177, 111], OperandSize::Qword)
}

#[test]
fn vfmsub213pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 255, 170, 255], OperandSize::Dword)
}

#[test]
fn vfmsub213pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexed(ESI, ESI, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 205, 207, 170, 4, 118], OperandSize::Dword)
}

#[test]
fn vfmsub213pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexed(EAX, EDX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 245, 221, 170, 28, 208], OperandSize::Dword)
}

#[test]
fn vfmsub213pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 197, 251, 170, 209], OperandSize::Qword)
}

#[test]
fn vfmsub213pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM29)), operand3: Some(Indirect(RBX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 149, 197, 170, 43], OperandSize::Qword)
}

#[test]
fn vfmsub213pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM13)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Two, 112183502, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 149, 220, 170, 132, 71, 206, 200, 175, 6], OperandSize::Qword)
}

