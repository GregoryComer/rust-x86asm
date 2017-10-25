use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovzxwd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 51, 194], OperandSize::Dword)
}

#[test]
fn vpmovzxwd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Four, 741313768, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 51, 132, 176, 232, 140, 47, 44], OperandSize::Dword)
}

#[test]
fn vpmovzxwd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 51, 224], OperandSize::Qword)
}

#[test]
fn vpmovzxwd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(RSI, 1460837966, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 51, 182, 78, 158, 18, 87], OperandSize::Qword)
}

#[test]
fn vpmovzxwd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 51, 200], OperandSize::Dword)
}

#[test]
fn vpmovzxwd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Eight, 1378321449, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 51, 180, 223, 41, 132, 39, 82], OperandSize::Dword)
}

#[test]
fn vpmovzxwd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 51, 232], OperandSize::Qword)
}

#[test]
fn vpmovzxwd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Two, 60663693, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 51, 164, 88, 141, 167, 157, 3], OperandSize::Qword)
}

#[test]
fn vpmovzxwd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 142, 51, 203], OperandSize::Dword)
}

#[test]
fn vpmovzxwd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 143, 51, 34], OperandSize::Dword)
}

#[test]
fn vpmovzxwd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 130, 125, 142, 51, 207], OperandSize::Qword)
}

#[test]
fn vpmovzxwd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(XMM12)), operand2: Some(IndirectScaledIndexed(RDI, RDI, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 125, 141, 51, 36, 255], OperandSize::Qword)
}

#[test]
fn vpmovzxwd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 175, 51, 239], OperandSize::Dword)
}

#[test]
fn vpmovzxwd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexed(EAX, EBX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 173, 51, 20, 88], OperandSize::Dword)
}

#[test]
fn vpmovzxwd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(YMM29)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 125, 172, 51, 239], OperandSize::Qword)
}

#[test]
fn vpmovzxwd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledIndexed(RSI, RSI, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 172, 51, 36, 118], OperandSize::Qword)
}

#[test]
fn vpmovzxwd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 207, 51, 235], OperandSize::Dword)
}

#[test]
fn vpmovzxwd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectDisplaced(EAX, 1399581825, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 207, 51, 128, 129, 236, 107, 83], OperandSize::Dword)
}

#[test]
fn vpmovzxwd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(YMM27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 130, 125, 207, 51, 235], OperandSize::Qword)
}

#[test]
fn vpmovzxwd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(ZMM9)), operand2: Some(IndirectDisplaced(RDX, 585932078, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 125, 202, 51, 138, 46, 157, 236, 34], OperandSize::Qword)
}

