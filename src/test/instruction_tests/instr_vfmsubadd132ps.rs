use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsubadd132ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 151, 198], OperandSize::Dword)
}

#[test]
fn vfmsubadd132ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 933999155, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 151, 60, 85, 51, 178, 171, 55], OperandSize::Dword)
}

#[test]
fn vfmsubadd132ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 151, 238], OperandSize::Qword)
}

#[test]
fn vfmsubadd132ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(RCX, RAX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 151, 44, 65], OperandSize::Qword)
}

#[test]
fn vfmsubadd132ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 151, 231], OperandSize::Dword)
}

#[test]
fn vfmsubadd132ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(ECX, EDX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 151, 28, 209], OperandSize::Dword)
}

#[test]
fn vfmsubadd132ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 151, 201], OperandSize::Qword)
}

#[test]
fn vfmsubadd132ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Two, 720520050, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 151, 132, 91, 114, 67, 242, 42], OperandSize::Qword)
}

#[test]
fn vfmsubadd132ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 142, 151, 230], OperandSize::Dword)
}

#[test]
fn vfmsubadd132ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 1082592820, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 69, 142, 151, 20, 205, 52, 14, 135, 64], OperandSize::Dword)
}

#[test]
fn vfmsubadd132ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(EAX, 2047799119, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 85, 159, 151, 160, 79, 239, 14, 122], OperandSize::Dword)
}

#[test]
fn vfmsubadd132ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM20)), operand3: Some(Direct(XMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 178, 93, 132, 151, 239], OperandSize::Qword)
}

#[test]
fn vfmsubadd132ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM19)), operand3: Some(IndirectScaledIndexed(RDX, RCX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 101, 134, 151, 28, 138], OperandSize::Qword)
}

#[test]
fn vfmsubadd132ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM28)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Four, 1011519452, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 29, 145, 151, 164, 191, 220, 143, 74, 60], OperandSize::Qword)
}

#[test]
fn vfmsubadd132ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 174, 151, 231], OperandSize::Dword)
}

#[test]
fn vfmsubadd132ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(EDX, 442366511, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 117, 172, 151, 162, 47, 250, 93, 26], OperandSize::Dword)
}

#[test]
fn vfmsubadd132ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 77, 190, 151, 31], OperandSize::Dword)
}

#[test]
fn vfmsubadd132ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM20)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 93, 165, 151, 202], OperandSize::Qword)
}

#[test]
fn vfmsubadd132ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM25)), operand3: Some(IndirectScaledIndexed(RSI, RBX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 53, 166, 151, 12, 158], OperandSize::Qword)
}

#[test]
fn vfmsubadd132ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM25)), operand3: Some(IndirectDisplaced(RBX, 563079837, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 53, 183, 151, 147, 157, 234, 143, 33], OperandSize::Qword)
}

#[test]
fn vfmsubadd132ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 109, 191, 151, 205], OperandSize::Dword)
}

#[test]
fn vfmsubadd132ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexed(EAX, ESI, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 117, 207, 151, 20, 112], OperandSize::Dword)
}

#[test]
fn vfmsubadd132ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 1647200493, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 77, 217, 151, 60, 133, 237, 72, 46, 98], OperandSize::Dword)
}

#[test]
fn vfmsubadd132ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM20)), operand3: Some(Direct(ZMM27)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 130, 93, 145, 151, 219], OperandSize::Qword)
}

#[test]
fn vfmsubadd132ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM28)), operand3: Some(IndirectDisplaced(RSI, 1910576753, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 29, 195, 151, 142, 113, 22, 225, 113], OperandSize::Qword)
}

#[test]
fn vfmsubadd132ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM25)), operand3: Some(IndirectDisplaced(RAX, 806485515, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 53, 211, 151, 160, 11, 254, 17, 48], OperandSize::Qword)
}

