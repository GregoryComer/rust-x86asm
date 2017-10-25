use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vprold_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(99)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 109, 140, 114, 201, 99], OperandSize::Dword)
}

#[test]
fn vprold_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(EAX, 719427554, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(83)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 77, 143, 114, 136, 226, 151, 225, 42, 83], OperandSize::Dword)
}

#[test]
fn vprold_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(ECX, Four, 1964023915, Some(OperandSize::Dword), None)), operand3: Some(Literal8(113)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 77, 159, 114, 12, 141, 107, 160, 16, 117, 113], OperandSize::Dword)
}

#[test]
fn vprold_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM21)), operand3: Some(Literal8(126)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 177, 109, 134, 114, 205, 126], OperandSize::Qword)
}

#[test]
fn vprold_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(XMM27)), operand2: Some(IndirectScaledDisplaced(RAX, Two, 1908620360, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(125)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 37, 132, 114, 12, 69, 72, 60, 195, 113, 125], OperandSize::Qword)
}

#[test]
fn vprold_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(XMM26)), operand2: Some(IndirectScaledDisplaced(RDI, Two, 1279533552, Some(OperandSize::Dword), None)), operand3: Some(Literal8(78)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 45, 146, 114, 12, 125, 240, 33, 68, 76, 78], OperandSize::Qword)
}

#[test]
fn vprold_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 85, 175, 114, 205, 19], OperandSize::Dword)
}

#[test]
fn vprold_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledDisplaced(ECX, Four, 1217396861, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(38)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 117, 174, 114, 12, 141, 125, 0, 144, 72, 38], OperandSize::Dword)
}

#[test]
fn vprold_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 47738997, Some(OperandSize::Dword), None)), operand3: Some(Literal8(75)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 101, 190, 114, 12, 117, 117, 112, 216, 2, 75], OperandSize::Dword)
}

#[test]
fn vprold_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM20)), operand3: Some(Literal8(116)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 177, 125, 163, 114, 204, 116], OperandSize::Qword)
}

#[test]
fn vprold_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 1712432236, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 170, 114, 12, 85, 108, 164, 17, 102, 27], OperandSize::Qword)
}

#[test]
fn vprold_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(YMM10)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Two, 488018055, Some(OperandSize::Dword), None)), operand3: Some(Literal8(59)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 45, 186, 114, 140, 71, 135, 144, 22, 29, 59], OperandSize::Qword)
}

#[test]
fn vprold_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM7)), operand3: Some(Literal8(41)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 85, 207, 114, 207, 41], OperandSize::Dword)
}

#[test]
fn vprold_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectDisplaced(EDI, 866558614, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(108)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 109, 202, 114, 143, 150, 162, 166, 51, 108], OperandSize::Dword)
}

#[test]
fn vprold_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectScaledDisplaced(EBX, Eight, 1773405767, Some(OperandSize::Dword), None)), operand3: Some(Literal8(65)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 69, 218, 114, 12, 221, 71, 6, 180, 105, 65], OperandSize::Dword)
}

#[test]
fn vprold_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM7)), operand3: Some(Literal8(80)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 61, 203, 114, 207, 80], OperandSize::Qword)
}

#[test]
fn vprold_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(ZMM8)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Eight, 1073360890, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(68)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 61, 202, 114, 140, 201, 250, 47, 250, 63, 68], OperandSize::Qword)
}

#[test]
fn vprold_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(ZMM8)), operand2: Some(IndirectScaledIndexed(RCX, RCX, Four, Some(OperandSize::Dword), None)), operand3: Some(Literal8(76)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 61, 220, 114, 12, 137, 76], OperandSize::Qword)
}

