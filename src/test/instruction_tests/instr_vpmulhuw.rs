use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmulhuw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 228, 218], OperandSize::Dword)
}

#[test]
fn vpmulhuw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 228, 59], OperandSize::Dword)
}

#[test]
fn vpmulhuw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 228, 255], OperandSize::Qword)
}

#[test]
fn vpmulhuw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 228, 15], OperandSize::Qword)
}

#[test]
fn vpmulhuw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 228, 206], OperandSize::Dword)
}

#[test]
fn vpmulhuw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 228, 22], OperandSize::Dword)
}

#[test]
fn vpmulhuw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 228, 219], OperandSize::Qword)
}

#[test]
fn vpmulhuw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 1956879927, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 228, 12, 157, 55, 158, 163, 116], OperandSize::Qword)
}

#[test]
fn vpmulhuw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 85, 139, 228, 240], OperandSize::Dword)
}

#[test]
fn vpmulhuw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 1495647910, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 85, 139, 228, 4, 69, 166, 198, 37, 89], OperandSize::Dword)
}

#[test]
fn vpmulhuw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 145, 101, 143, 228, 216], OperandSize::Qword)
}

#[test]
fn vpmulhuw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 69, 137, 228, 47], OperandSize::Qword)
}

#[test]
fn vpmulhuw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 117, 173, 228, 253], OperandSize::Dword)
}

#[test]
fn vpmulhuw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(EBX, EDI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 85, 171, 228, 4, 187], OperandSize::Dword)
}

#[test]
fn vpmulhuw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM12)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 29, 175, 228, 246], OperandSize::Qword)
}

#[test]
fn vpmulhuw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM25)), operand3: Some(IndirectDisplaced(RDI, 770112087, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 53, 164, 228, 183, 87, 250, 230, 45], OperandSize::Qword)
}

#[test]
fn vpmulhuw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 117, 203, 228, 224], OperandSize::Dword)
}

#[test]
fn vpmulhuw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 1195298764, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 85, 205, 228, 12, 157, 204, 207, 62, 71], OperandSize::Dword)
}

#[test]
fn vpmulhuw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM27)), operand3: Some(Direct(ZMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 129, 37, 195, 228, 229], OperandSize::Qword)
}

#[test]
fn vpmulhuw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(ZMM24)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Two, 1928449725, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 125, 203, 228, 132, 119, 189, 206, 241, 114], OperandSize::Qword)
}

