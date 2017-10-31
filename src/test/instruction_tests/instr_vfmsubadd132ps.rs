use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsubadd132ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 151, 231], OperandSize::Dword)
}

#[test]
fn vfmsubadd132ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(ECX, EDX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 151, 4, 209], OperandSize::Dword)
}

#[test]
fn vfmsubadd132ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 151, 220], OperandSize::Qword)
}

#[test]
fn vfmsubadd132ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(RDI, 2129416772, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 151, 159, 68, 82, 236, 126], OperandSize::Qword)
}

#[test]
fn vfmsubadd132ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 151, 208], OperandSize::Dword)
}

#[test]
fn vfmsubadd132ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 945023111, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 151, 28, 117, 135, 232, 83, 56], OperandSize::Dword)
}

#[test]
fn vfmsubadd132ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 151, 229], OperandSize::Qword)
}

#[test]
fn vfmsubadd132ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(RBX, RDI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 151, 44, 187], OperandSize::Qword)
}

#[test]
fn vfmsubadd132ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 69, 138, 151, 235], OperandSize::Dword)
}

#[test]
fn vfmsubadd132ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(EDI, EAX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 69, 143, 151, 20, 71], OperandSize::Dword)
}

#[test]
fn vfmsubadd132ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 693865143, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 125, 154, 151, 20, 181, 183, 138, 91, 41], OperandSize::Dword)
}

#[test]
fn vfmsubadd132ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 130, 101, 142, 151, 254], OperandSize::Qword)
}

#[test]
fn vfmsubadd132ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM24)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Four, 973527182, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 61, 131, 151, 132, 159, 142, 216, 6, 58], OperandSize::Qword)
}

#[test]
fn vfmsubadd132ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Two, 1384553743, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 77, 159, 151, 140, 87, 15, 157, 134, 82], OperandSize::Qword)
}

#[test]
fn vfmsubadd132ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 117, 174, 151, 225], OperandSize::Dword)
}

#[test]
fn vfmsubadd132ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 101, 174, 151, 51], OperandSize::Dword)
}

#[test]
fn vfmsubadd132ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Eight, 592170164, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 117, 189, 151, 188, 254, 180, 204, 75, 35], OperandSize::Dword)
}

#[test]
fn vfmsubadd132ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM12)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 29, 169, 151, 233], OperandSize::Qword)
}

#[test]
fn vfmsubadd132ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM9)), operand3: Some(IndirectDisplaced(RAX, 1523623892, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 53, 174, 151, 160, 212, 167, 208, 90], OperandSize::Qword)
}

#[test]
fn vfmsubadd132ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM28)), operand3: Some(IndirectDisplaced(RBX, 1012963925, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 29, 178, 151, 131, 85, 154, 96, 60], OperandSize::Qword)
}

#[test]
fn vfmsubadd132ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 109, 155, 151, 206], OperandSize::Dword)
}

#[test]
fn vfmsubadd132ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM0)), operand3: Some(Indirect(EBX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 203, 151, 27], OperandSize::Dword)
}

#[test]
fn vfmsubadd132ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 2032447593, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 69, 221, 151, 12, 197, 105, 176, 36, 121], OperandSize::Dword)
}

#[test]
fn vfmsubadd132ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM22)), operand3: Some(Direct(ZMM9)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 66, 77, 242, 151, 209], OperandSize::Qword)
}

#[test]
fn vfmsubadd132ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM22)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 723528577, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 77, 193, 151, 36, 133, 129, 43, 32, 43], OperandSize::Qword)
}

#[test]
fn vfmsubadd132ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Two, 1960803757, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 101, 221, 151, 156, 113, 173, 125, 223, 116], OperandSize::Qword)
}

