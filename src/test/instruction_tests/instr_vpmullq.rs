use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmullq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 205, 138, 64, 235], OperandSize::Dword)
}

#[test]
fn vpmullq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(EBX, 331673470, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 137, 64, 147, 126, 239, 196, 19], OperandSize::Dword)
}

#[test]
fn vpmullq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 253, 157, 64, 23], OperandSize::Dword)
}

#[test]
fn vpmullq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM18)), operand3: Some(Direct(XMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 50, 237, 132, 64, 205], OperandSize::Qword)
}

#[test]
fn vpmullq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM19)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Eight, 1760917891, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 229, 129, 64, 132, 215, 131, 121, 245, 104], OperandSize::Qword)
}

#[test]
fn vpmullq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM13)), operand3: Some(IndirectDisplaced(RDI, 890192359, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 114, 149, 157, 64, 175, 231, 65, 15, 53], OperandSize::Qword)
}

#[test]
fn vpmullq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 172, 64, 193], OperandSize::Dword)
}

#[test]
fn vpmullq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 1349886561, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 213, 173, 64, 20, 149, 97, 162, 117, 80], OperandSize::Dword)
}

#[test]
fn vpmullq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Four, 1767973783, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 253, 188, 64, 180, 191, 151, 35, 97, 105], OperandSize::Dword)
}

#[test]
fn vpmullq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM21)), operand3: Some(Direct(YMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 82, 213, 164, 64, 232], OperandSize::Qword)
}

#[test]
fn vpmullq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM23)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 513182060, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 197, 164, 64, 52, 77, 108, 137, 150, 30], OperandSize::Qword)
}

#[test]
fn vpmullq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM13)), operand3: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 149, 191, 64, 17], OperandSize::Qword)
}

#[test]
fn vpmullq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 207, 64, 198], OperandSize::Dword)
}

#[test]
fn vpmullq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Eight, 1204304785, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 213, 201, 64, 156, 219, 145, 59, 200, 71], OperandSize::Dword)
}

#[test]
fn vpmullq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Four, 1159217553, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 217, 64, 172, 185, 145, 65, 24, 69], OperandSize::Dword)
}

#[test]
fn vpmullq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM31)), operand3: Some(Direct(ZMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 34, 133, 196, 64, 240], OperandSize::Qword)
}

#[test]
fn vpmullq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM29)), operand3: Some(IndirectDisplaced(RBX, 716469926, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 149, 197, 64, 187, 166, 118, 180, 42], OperandSize::Qword)
}

#[test]
fn vpmullq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM14)), operand3: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 141, 222, 64, 58], OperandSize::Qword)
}

