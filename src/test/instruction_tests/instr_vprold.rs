use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vprold_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(33)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 117, 141, 114, 207, 33], OperandSize::Dword)
}

#[test]
fn vprold_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(EDI, ESI, Eight, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(52)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 138, 114, 12, 247, 52], OperandSize::Dword)
}

#[test]
fn vprold_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(EBX, 1299992953, Some(OperandSize::Dword), None)), operand3: Some(Literal8(58)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 93, 159, 114, 139, 121, 81, 124, 77, 58], OperandSize::Dword)
}

#[test]
fn vprold_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM24)), operand3: Some(Literal8(42)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 145, 125, 131, 114, 200, 42], OperandSize::Qword)
}

#[test]
fn vprold_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(XMM27)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Four, 1099791945, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 37, 129, 114, 140, 178, 73, 126, 141, 65, 25], OperandSize::Qword)
}

#[test]
fn vprold_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(XMM12)), operand2: Some(IndirectScaledIndexed(RSI, RSI, Eight, Some(OperandSize::Dword), None)), operand3: Some(Literal8(106)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 29, 157, 114, 12, 246, 106], OperandSize::Qword)
}

#[test]
fn vprold_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(Literal8(59)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 69, 169, 114, 201, 59], OperandSize::Dword)
}

#[test]
fn vprold_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 1857468285, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 109, 173, 114, 12, 77, 125, 183, 182, 110, 13], OperandSize::Dword)
}

#[test]
fn vprold_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(YMM3)), operand2: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand3: Some(Literal8(125)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 101, 189, 114, 15, 125], OperandSize::Dword)
}

#[test]
fn vprold_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM28)), operand3: Some(Literal8(103)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 145, 109, 165, 114, 204, 103], OperandSize::Qword)
}

#[test]
fn vprold_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledDisplaced(RAX, Eight, 280284984, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(109)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 85, 170, 114, 12, 197, 56, 207, 180, 16, 109], OperandSize::Qword)
}

#[test]
fn vprold_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(YMM23)), operand2: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand3: Some(Literal8(65)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 69, 181, 114, 9, 65], OperandSize::Qword)
}

#[test]
fn vprold_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM6)), operand3: Some(Literal8(34)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 117, 203, 114, 206, 34], OperandSize::Dword)
}

#[test]
fn vprold_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Four, 122147949, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(111)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 85, 206, 114, 140, 152, 109, 212, 71, 7, 111], OperandSize::Dword)
}

#[test]
fn vprold_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(ZMM6)), operand2: Some(IndirectDisplaced(EDI, 48615894, Some(OperandSize::Dword), None)), operand3: Some(Literal8(2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 77, 218, 114, 143, 214, 209, 229, 2, 2], OperandSize::Dword)
}

#[test]
fn vprold_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM22)), operand3: Some(Literal8(14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 177, 21, 204, 114, 206, 14], OperandSize::Qword)
}

#[test]
fn vprold_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(ZMM8)), operand2: Some(IndirectDisplaced(RBX, 261722750, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(68)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 61, 205, 114, 139, 126, 146, 153, 15, 68], OperandSize::Qword)
}

#[test]
fn vprold_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(ZMM11)), operand2: Some(IndirectDisplaced(RDX, 344439124, Some(OperandSize::Dword), None)), operand3: Some(Literal8(88)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 37, 223, 114, 138, 84, 185, 135, 20, 88], OperandSize::Qword)
}

