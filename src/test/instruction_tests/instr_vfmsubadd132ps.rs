use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsubadd132ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 151, 229], OperandSize::Dword)
}

#[test]
fn vfmsubadd132ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(EBX, EDX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 151, 12, 211], OperandSize::Dword)
}

#[test]
fn vfmsubadd132ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 151, 241], OperandSize::Qword)
}

#[test]
fn vfmsubadd132ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Two, 1571927834, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 151, 180, 115, 26, 183, 177, 93], OperandSize::Qword)
}

#[test]
fn vfmsubadd132ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 151, 241], OperandSize::Dword)
}

#[test]
fn vfmsubadd132ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 151, 59], OperandSize::Dword)
}

#[test]
fn vfmsubadd132ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 151, 193], OperandSize::Qword)
}

#[test]
fn vfmsubadd132ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(RDI, 1890138986, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 151, 135, 106, 59, 169, 112], OperandSize::Qword)
}

#[test]
fn vfmsubadd132ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 85, 140, 151, 245], OperandSize::Dword)
}

#[test]
fn vfmsubadd132ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(ECX, EAX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 142, 151, 28, 65], OperandSize::Dword)
}

#[test]
fn vfmsubadd132ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(EBX, 399325509, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 109, 154, 151, 179, 69, 57, 205, 23], OperandSize::Dword)
}

#[test]
fn vfmsubadd132ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM8)), operand3: Some(Direct(XMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 50, 61, 137, 151, 210], OperandSize::Qword)
}

#[test]
fn vfmsubadd132ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM25)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Two, 1298693962, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 53, 131, 151, 156, 71, 74, 127, 104, 77], OperandSize::Qword)
}

#[test]
fn vfmsubadd132ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(RAX, RDX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 109, 158, 151, 12, 80], OperandSize::Qword)
}

#[test]
fn vfmsubadd132ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 85, 171, 151, 201], OperandSize::Dword)
}

#[test]
fn vfmsubadd132ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 295835724, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 109, 173, 151, 4, 213, 76, 24, 162, 17], OperandSize::Dword)
}

#[test]
fn vfmsubadd132ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 93, 191, 151, 41], OperandSize::Dword)
}

#[test]
fn vfmsubadd132ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM14)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 13, 172, 151, 252], OperandSize::Qword)
}

#[test]
fn vfmsubadd132ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(RCX, 298378101, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 85, 171, 151, 129, 117, 227, 200, 17], OperandSize::Qword)
}

#[test]
fn vfmsubadd132ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM24)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Two, 1381758135, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 61, 177, 151, 140, 118, 183, 244, 91, 82], OperandSize::Qword)
}

#[test]
fn vfmsubadd132ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 101, 155, 151, 227], OperandSize::Dword)
}

#[test]
fn vfmsubadd132ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 528294448, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 109, 202, 151, 4, 85, 48, 34, 125, 31], OperandSize::Dword)
}

#[test]
fn vfmsubadd132ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Eight, 186773194, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 93, 221, 151, 172, 207, 202, 238, 33, 11], OperandSize::Dword)
}

#[test]
fn vfmsubadd132ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM17)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 117, 212, 151, 221], OperandSize::Qword)
}

#[test]
fn vfmsubadd132ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM10)), operand3: Some(Indirect(RBX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 45, 205, 151, 59], OperandSize::Qword)
}

#[test]
fn vfmsubadd132ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM29)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Four, 390603763, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 226, 21, 215, 151, 172, 139, 243, 35, 72, 23], OperandSize::Qword)
}

