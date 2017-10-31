use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsubadd231pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 183, 201], OperandSize::Dword)
}

#[test]
fn vfmsubadd231pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 183, 15], OperandSize::Dword)
}

#[test]
fn vfmsubadd231pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 183, 196], OperandSize::Qword)
}

#[test]
fn vfmsubadd231pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Two, 230086981, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 183, 180, 112, 69, 217, 182, 13], OperandSize::Qword)
}

#[test]
fn vfmsubadd231pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 237, 183, 226], OperandSize::Dword)
}

#[test]
fn vfmsubadd231pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 229, 183, 10], OperandSize::Dword)
}

#[test]
fn vfmsubadd231pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 237, 183, 228], OperandSize::Qword)
}

#[test]
fn vfmsubadd231pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 2047060605, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 253, 183, 52, 245, 125, 170, 3, 122], OperandSize::Qword)
}

#[test]
fn vfmsubadd231pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 197, 140, 183, 208], OperandSize::Dword)
}

#[test]
fn vfmsubadd231pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Eight, 1853710740, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 205, 137, 183, 156, 214, 148, 97, 125, 110], OperandSize::Dword)
}

#[test]
fn vfmsubadd231pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 1269648109, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 205, 159, 183, 52, 181, 237, 74, 173, 75], OperandSize::Dword)
}

#[test]
fn vfmsubadd231pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM30)), operand3: Some(Direct(XMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 130, 141, 135, 183, 205], OperandSize::Qword)
}

#[test]
fn vfmsubadd231pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM30)), operand3: Some(IndirectDisplaced(RAX, 1548665575, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 141, 133, 183, 128, 231, 194, 78, 92], OperandSize::Qword)
}

#[test]
fn vfmsubadd231pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM23)), operand3: Some(IndirectDisplaced(RDI, 1091094148, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 98, 197, 146, 183, 143, 132, 198, 8, 65], OperandSize::Qword)
}

#[test]
fn vfmsubadd231pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 205, 173, 183, 203], OperandSize::Dword)
}

#[test]
fn vfmsubadd231pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Eight, 1626500626, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 237, 169, 183, 180, 222, 18, 110, 242, 96], OperandSize::Dword)
}

#[test]
fn vfmsubadd231pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 221, 191, 183, 19], OperandSize::Dword)
}

#[test]
fn vfmsubadd231pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM15)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 133, 174, 183, 221], OperandSize::Qword)
}

#[test]
fn vfmsubadd231pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM18)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Eight, 1225330454, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 237, 163, 183, 156, 195, 22, 15, 9, 73], OperandSize::Qword)
}

#[test]
fn vfmsubadd231pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 1838617327, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 213, 191, 183, 20, 253, 239, 18, 151, 109], OperandSize::Qword)
}

#[test]
fn vfmsubadd231pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 245, 253, 183, 241], OperandSize::Dword)
}

#[test]
fn vfmsubadd231pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexed(EAX, EDX, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 205, 201, 183, 20, 80], OperandSize::Dword)
}

#[test]
fn vfmsubadd231pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM5)), operand3: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 213, 220, 183, 1], OperandSize::Dword)
}

#[test]
fn vfmsubadd231pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM21)), operand3: Some(Direct(ZMM26)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 18, 213, 147, 183, 210], OperandSize::Qword)
}

#[test]
fn vfmsubadd231pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM10)), operand3: Some(Indirect(RSI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 173, 205, 183, 62], OperandSize::Qword)
}

#[test]
fn vfmsubadd231pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectDisplaced(RDI, 1472793350, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 253, 219, 183, 167, 6, 11, 201, 87], OperandSize::Qword)
}

