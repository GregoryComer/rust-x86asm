use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vprord_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(94)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 117, 138, 114, 199, 94], OperandSize::Dword)
}

#[test]
fn vprord_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 77, 141, 114, 3, 5], OperandSize::Dword)
}

#[test]
fn vprord_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Eight, 1591803564, Some(OperandSize::Dword), None)), operand3: Some(Literal8(83)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 117, 153, 114, 132, 255, 172, 254, 224, 94, 83], OperandSize::Dword)
}

#[test]
fn vprord_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(75)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 69, 133, 114, 196, 75], OperandSize::Qword)
}

#[test]
fn vprord_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Two, 967758761, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 109, 142, 114, 132, 67, 169, 211, 174, 57, 27], OperandSize::Qword)
}

#[test]
fn vprord_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(XMM14)), operand2: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand3: Some(Literal8(90)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 13, 157, 114, 3, 90], OperandSize::Qword)
}

#[test]
fn vprord_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(Literal8(97)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 93, 173, 114, 199, 97], OperandSize::Dword)
}

#[test]
fn vprord_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Two, 812853127, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(50)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 85, 175, 114, 132, 71, 135, 39, 115, 48, 50], OperandSize::Dword)
}

#[test]
fn vprord_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Two, 222464644, Some(OperandSize::Dword), None)), operand3: Some(Literal8(52)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 125, 190, 114, 132, 122, 132, 138, 66, 13, 52], OperandSize::Dword)
}

#[test]
fn vprord_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM18)), operand3: Some(Literal8(77)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 177, 117, 161, 114, 194, 77], OperandSize::Qword)
}

#[test]
fn vprord_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 1226455295, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(119)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 101, 169, 114, 4, 117, 255, 56, 26, 73, 119], OperandSize::Qword)
}

#[test]
fn vprord_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(YMM14)), operand2: Some(IndirectDisplaced(RDX, 1736278189, Some(OperandSize::Dword), None)), operand3: Some(Literal8(45)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 13, 185, 114, 130, 173, 128, 125, 103, 45], OperandSize::Qword)
}

#[test]
fn vprord_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM3)), operand3: Some(Literal8(23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 77, 207, 114, 195, 23], OperandSize::Dword)
}

#[test]
fn vprord_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledIndexed(ESI, EDX, Four, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(62)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 85, 204, 114, 4, 150, 62], OperandSize::Dword)
}

#[test]
fn vprord_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectScaledDisplaced(ECX, Eight, 1425261059, Some(OperandSize::Dword), None)), operand3: Some(Literal8(33)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 69, 223, 114, 4, 205, 3, 194, 243, 84, 33], OperandSize::Dword)
}

#[test]
fn vprord_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM8)), operand3: Some(Literal8(51)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 209, 13, 195, 114, 192, 51], OperandSize::Qword)
}

#[test]
fn vprord_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(ZMM6)), operand2: Some(IndirectScaledIndexed(RCX, RDI, Eight, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(111)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 77, 207, 114, 4, 249, 111], OperandSize::Qword)
}

#[test]
fn vprord_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(ZMM19)), operand2: Some(IndirectScaledDisplaced(RCX, Two, 2125217830, Some(OperandSize::Dword), None)), operand3: Some(Literal8(114)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 101, 209, 114, 4, 77, 38, 64, 172, 126, 114], OperandSize::Qword)
}

