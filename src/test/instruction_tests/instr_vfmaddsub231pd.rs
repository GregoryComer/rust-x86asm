use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmaddsub231pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 182, 218], OperandSize::Dword)
}

#[test]
fn vfmaddsub231pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 182, 31], OperandSize::Dword)
}

#[test]
fn vfmaddsub231pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 182, 223], OperandSize::Qword)
}

#[test]
fn vfmaddsub231pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 182, 40], OperandSize::Qword)
}

#[test]
fn vfmaddsub231pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 229, 182, 227], OperandSize::Dword)
}

#[test]
fn vfmaddsub231pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 253, 182, 7], OperandSize::Dword)
}

#[test]
fn vfmaddsub231pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 253, 182, 212], OperandSize::Qword)
}

#[test]
fn vfmaddsub231pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(RBX, RSI, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 205, 182, 44, 243], OperandSize::Qword)
}

#[test]
fn vfmaddsub231pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 237, 137, 182, 200], OperandSize::Dword)
}

#[test]
fn vfmaddsub231pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 229, 138, 182, 9], OperandSize::Dword)
}

#[test]
fn vfmaddsub231pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Two, 1266539554, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 205, 158, 182, 156, 91, 34, 220, 125, 75], OperandSize::Dword)
}

#[test]
fn vfmaddsub231pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM31)), operand3: Some(Direct(XMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 146, 133, 135, 182, 196], OperandSize::Qword)
}

#[test]
fn vfmaddsub231pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM29)), operand3: Some(IndirectScaledIndexed(RSI, RDX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 149, 132, 182, 28, 150], OperandSize::Qword)
}

#[test]
fn vfmaddsub231pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM25)), operand3: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 114, 181, 145, 182, 35], OperandSize::Qword)
}

#[test]
fn vfmaddsub231pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 213, 173, 182, 193], OperandSize::Dword)
}

#[test]
fn vfmaddsub231pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(ECX, 438343412, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 229, 169, 182, 169, 244, 150, 32, 26], OperandSize::Dword)
}

#[test]
fn vfmaddsub231pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 1799811006, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 229, 186, 182, 52, 221, 190, 239, 70, 107], OperandSize::Dword)
}

#[test]
fn vfmaddsub231pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM10)), operand3: Some(Direct(YMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 50, 173, 172, 182, 219], OperandSize::Qword)
}

#[test]
fn vfmaddsub231pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM31)), operand3: Some(IndirectDisplaced(RCX, 1387170981, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 133, 164, 182, 177, 165, 140, 174, 82], OperandSize::Qword)
}

#[test]
fn vfmaddsub231pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM22)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Four, 2143080065, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 205, 177, 182, 140, 190, 129, 206, 188, 127], OperandSize::Qword)
}

#[test]
fn vfmaddsub231pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 223, 182, 226], OperandSize::Dword)
}

#[test]
fn vfmaddsub231pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM2)), operand3: Some(Indirect(EDI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 237, 202, 182, 47], OperandSize::Dword)
}

#[test]
fn vfmaddsub231pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexed(ECX, EAX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 229, 217, 182, 52, 65], OperandSize::Dword)
}

#[test]
fn vfmaddsub231pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM20)), operand3: Some(Direct(ZMM24)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 18, 221, 179, 182, 200], OperandSize::Qword)
}

#[test]
fn vfmaddsub231pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM31)), operand3: Some(IndirectDisplaced(RDX, 1447472216, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 133, 199, 182, 186, 88, 172, 70, 86], OperandSize::Qword)
}

#[test]
fn vfmaddsub231pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM23)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Four, 1977565267, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 197, 214, 182, 140, 135, 83, 64, 223, 117], OperandSize::Qword)
}

