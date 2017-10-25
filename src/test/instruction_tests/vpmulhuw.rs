use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpmulhuw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 228, 243], OperandSize::Dword)
}

fn vpmulhuw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 228, 10], OperandSize::Dword)
}

fn vpmulhuw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 228, 244], OperandSize::Qword)
}

fn vpmulhuw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(RCX, RBX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 228, 36, 153], OperandSize::Qword)
}

fn vpmulhuw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 228, 214], OperandSize::Dword)
}

fn vpmulhuw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Four, 1745914130, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 228, 132, 130, 18, 137, 16, 104], OperandSize::Dword)
}

fn vpmulhuw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 228, 209], OperandSize::Qword)
}

fn vpmulhuw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 228, 30], OperandSize::Qword)
}

fn vpmulhuw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 109, 140, 228, 241], OperandSize::Dword)
}

fn vpmulhuw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(EAX, EDX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 125, 141, 228, 12, 144], OperandSize::Dword)
}

fn vpmulhuw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 49, 101, 137, 228, 234], OperandSize::Qword)
}

fn vpmulhuw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM12)), operand3: Some(IndirectScaledIndexed(RAX, RDI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 29, 143, 228, 44, 248], OperandSize::Qword)
}

fn vpmulhuw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 77, 175, 228, 246], OperandSize::Dword)
}

fn vpmulhuw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(ECX, 1369549098, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 175, 228, 129, 42, 169, 161, 81], OperandSize::Dword)
}

fn vpmulhuw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM11)), operand3: Some(Direct(YMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 17, 37, 174, 228, 245], OperandSize::Qword)
}

fn vpmulhuw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 109, 173, 228, 23], OperandSize::Qword)
}

fn vpmulhuw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 125, 205, 228, 231], OperandSize::Dword)
}

fn vpmulhuw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexed(EDX, EDX, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 85, 205, 228, 12, 210], OperandSize::Dword)
}

fn vpmulhuw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM29)), operand3: Some(Direct(ZMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 193, 21, 199, 228, 212], OperandSize::Qword)
}

fn vpmulhuw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(ZMM24)), operand2: Some(Direct(ZMM16)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 2121770746, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 125, 194, 228, 4, 125, 250, 166, 119, 126], OperandSize::Qword)
}

