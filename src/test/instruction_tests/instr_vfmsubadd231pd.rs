use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsubadd231pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 183, 202], OperandSize::Dword)
}

#[test]
fn vfmsubadd231pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 183, 0], OperandSize::Dword)
}

#[test]
fn vfmsubadd231pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 183, 224], OperandSize::Qword)
}

#[test]
fn vfmsubadd231pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(RAX, 630521228, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 183, 160, 140, 253, 148, 37], OperandSize::Qword)
}

#[test]
fn vfmsubadd231pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 197, 183, 206], OperandSize::Dword)
}

#[test]
fn vfmsubadd231pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 197, 183, 59], OperandSize::Dword)
}

#[test]
fn vfmsubadd231pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 213, 183, 205], OperandSize::Qword)
}

#[test]
fn vfmsubadd231pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(RDX, 613397265, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 253, 183, 178, 17, 179, 143, 36], OperandSize::Qword)
}

#[test]
fn vfmsubadd231pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 237, 141, 183, 215], OperandSize::Dword)
}

#[test]
fn vfmsubadd231pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 142, 183, 51], OperandSize::Dword)
}

#[test]
fn vfmsubadd231pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(EDX, ECX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 197, 153, 183, 12, 138], OperandSize::Dword)
}

#[test]
fn vfmsubadd231pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM29)), operand3: Some(Direct(XMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 50, 149, 129, 183, 193], OperandSize::Qword)
}

#[test]
fn vfmsubadd231pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM15)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 133, 142, 183, 50], OperandSize::Qword)
}

#[test]
fn vfmsubadd231pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM27)), operand3: Some(IndirectDisplaced(RDI, 1864223042, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 114, 165, 147, 183, 159, 66, 201, 29, 111], OperandSize::Qword)
}

#[test]
fn vfmsubadd231pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 237, 169, 183, 222], OperandSize::Dword)
}

#[test]
fn vfmsubadd231pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(ECX, ECX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 197, 172, 183, 12, 73], OperandSize::Dword)
}

#[test]
fn vfmsubadd231pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(ECX, 1696199781, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 237, 186, 183, 153, 101, 244, 25, 101], OperandSize::Dword)
}

#[test]
fn vfmsubadd231pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM14)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 141, 173, 183, 219], OperandSize::Qword)
}

#[test]
fn vfmsubadd231pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 114417901, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 213, 173, 183, 44, 221, 237, 224, 209, 6], OperandSize::Qword)
}

#[test]
fn vfmsubadd231pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM8)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 447536457, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 189, 191, 183, 12, 245, 73, 221, 172, 26], OperandSize::Qword)
}

#[test]
fn vfmsubadd231pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 213, 186, 183, 229], OperandSize::Dword)
}

#[test]
fn vfmsubadd231pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM3)), operand3: Some(Indirect(EBX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 229, 205, 183, 3], OperandSize::Dword)
}

#[test]
fn vfmsubadd231pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexed(EAX, EAX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 229, 220, 183, 12, 192], OperandSize::Dword)
}

#[test]
fn vfmsubadd231pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM8)), operand3: Some(Direct(ZMM28)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 146, 189, 158, 183, 204], OperandSize::Qword)
}

#[test]
fn vfmsubadd231pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM9)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Two, 194503505, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 181, 202, 183, 148, 81, 81, 227, 151, 11], OperandSize::Qword)
}

#[test]
fn vfmsubadd231pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM20)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 6298887, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 221, 214, 183, 52, 133, 7, 29, 96, 0], OperandSize::Qword)
}

