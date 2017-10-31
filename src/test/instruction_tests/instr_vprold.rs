use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vprold_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 101, 141, 114, 205, 8], OperandSize::Dword)
}

#[test]
fn vprold_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 1525668111, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 69, 137, 114, 12, 69, 15, 217, 239, 90, 17], OperandSize::Dword)
}

#[test]
fn vprold_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(ECX, Eight, 1515750872, Some(OperandSize::Dword), None)), operand3: Some(Literal8(113)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 93, 156, 114, 12, 205, 216, 133, 88, 90, 113], OperandSize::Dword)
}

#[test]
fn vprold_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM18)), operand3: Some(Literal8(51)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 177, 37, 132, 114, 202, 51], OperandSize::Qword)
}

#[test]
fn vprold_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(XMM17)), operand2: Some(IndirectScaledIndexed(RAX, RDX, Two, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(32)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 117, 129, 114, 12, 80, 32], OperandSize::Qword)
}

#[test]
fn vprold_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(RDI, Four, 567752779, Some(OperandSize::Dword), None)), operand3: Some(Literal8(41)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 101, 157, 114, 12, 189, 75, 56, 215, 33, 41], OperandSize::Qword)
}

#[test]
fn vprold_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(Literal8(109)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 69, 173, 114, 204, 109], OperandSize::Dword)
}

#[test]
fn vprold_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(YMM7)), operand2: Some(IndirectDisplaced(EDX, 295205545, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(121)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 69, 171, 114, 138, 169, 122, 152, 17, 121], OperandSize::Dword)
}

#[test]
fn vprold_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledIndexed(ESI, EDI, Two, Some(OperandSize::Dword), None)), operand3: Some(Literal8(97)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 101, 190, 114, 12, 126, 97], OperandSize::Dword)
}

#[test]
fn vprold_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM13)), operand3: Some(Literal8(101)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 209, 117, 172, 114, 205, 101], OperandSize::Qword)
}

#[test]
fn vprold_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(YMM15)), operand2: Some(IndirectScaledIndexed(RSI, RSI, Four, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(93)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 5, 170, 114, 12, 182, 93], OperandSize::Qword)
}

#[test]
fn vprold_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 290002993, Some(OperandSize::Dword), None)), operand3: Some(Literal8(8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 93, 187, 114, 12, 245, 49, 24, 73, 17, 8], OperandSize::Qword)
}

#[test]
fn vprold_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM1)), operand3: Some(Literal8(65)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 85, 203, 114, 201, 65], OperandSize::Dword)
}

#[test]
fn vprold_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectScaledIndexed(ECX, ECX, Four, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 109, 204, 114, 12, 137, 21], OperandSize::Dword)
}

#[test]
fn vprold_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Four, 1668529594, Some(OperandSize::Dword), None)), operand3: Some(Literal8(67)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 109, 220, 114, 140, 130, 186, 189, 115, 99, 67], OperandSize::Dword)
}

#[test]
fn vprold_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM26)), operand3: Some(Literal8(12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 145, 125, 204, 114, 202, 12], OperandSize::Qword)
}

#[test]
fn vprold_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(ZMM27)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 1909258150, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(80)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 37, 194, 114, 12, 85, 166, 247, 204, 113, 80], OperandSize::Qword)
}

#[test]
fn vprold_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLD, operand1: Some(Direct(ZMM29)), operand2: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand3: Some(Literal8(41)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 21, 212, 114, 11, 41], OperandSize::Qword)
}

