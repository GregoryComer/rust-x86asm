use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsub213pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 170, 209], OperandSize::Dword)
}

#[test]
fn vfmsub213pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 170, 18], OperandSize::Dword)
}

#[test]
fn vfmsub213pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 170, 194], OperandSize::Qword)
}

#[test]
fn vfmsub213pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 170, 48], OperandSize::Qword)
}

#[test]
fn vfmsub213pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 213, 170, 209], OperandSize::Dword)
}

#[test]
fn vfmsub213pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Two, 793996301, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 197, 170, 132, 119, 13, 108, 83, 47], OperandSize::Dword)
}

#[test]
fn vfmsub213pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 205, 170, 197], OperandSize::Qword)
}

#[test]
fn vfmsub213pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Two, 1393366624, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 253, 170, 132, 95, 96, 22, 13, 83], OperandSize::Qword)
}

#[test]
fn vfmsub213pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 237, 141, 170, 194], OperandSize::Dword)
}

#[test]
fn vfmsub213pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Eight, 1852221309, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 245, 137, 170, 132, 210, 125, 167, 102, 110], OperandSize::Dword)
}

#[test]
fn vfmsub213pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(EDI, EAX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 253, 153, 170, 20, 199], OperandSize::Dword)
}

#[test]
fn vfmsub213pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM30)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 141, 135, 170, 231], OperandSize::Qword)
}

#[test]
fn vfmsub213pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM29)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 1538148186, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 149, 135, 170, 12, 141, 90, 71, 174, 91], OperandSize::Qword)
}

#[test]
fn vfmsub213pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(RSI, 599489816, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 98, 245, 159, 170, 150, 24, 125, 187, 35], OperandSize::Qword)
}

#[test]
fn vfmsub213pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 237, 173, 170, 199], OperandSize::Dword)
}

#[test]
fn vfmsub213pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 552114864, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 197, 174, 170, 52, 181, 176, 154, 232, 32], OperandSize::Dword)
}

#[test]
fn vfmsub213pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Four, 1880093891, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 229, 188, 170, 180, 158, 195, 244, 15, 112], OperandSize::Dword)
}

#[test]
fn vfmsub213pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 162, 213, 169, 170, 250], OperandSize::Qword)
}

#[test]
fn vfmsub213pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM31)), operand3: Some(IndirectDisplaced(RAX, 6082892, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 133, 167, 170, 184, 76, 209, 92, 0], OperandSize::Qword)
}

#[test]
fn vfmsub213pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM22)), operand3: Some(IndirectScaledIndexed(RDI, RDX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 205, 179, 170, 60, 215], OperandSize::Qword)
}

#[test]
fn vfmsub213pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 197, 155, 170, 213], OperandSize::Dword)
}

#[test]
fn vfmsub213pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexed(EAX, EAX, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 213, 204, 170, 60, 64], OperandSize::Dword)
}

#[test]
fn vfmsub213pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexed(EBX, ESI, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 229, 221, 170, 52, 115], OperandSize::Dword)
}

#[test]
fn vfmsub213pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM20)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 221, 145, 170, 223], OperandSize::Qword)
}

#[test]
fn vfmsub213pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM24)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Two, 1077959533, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 189, 196, 170, 140, 72, 109, 91, 64, 64], OperandSize::Qword)
}

#[test]
fn vfmsub213pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM27)), operand3: Some(IndirectDisplaced(RAX, 811081, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 165, 211, 170, 160, 73, 96, 12, 0], OperandSize::Qword)
}

