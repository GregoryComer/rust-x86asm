use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovzxbw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 48, 229], OperandSize::Dword)
}

#[test]
fn vpmovzxbw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Two, 680928705, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 48, 188, 86, 193, 37, 150, 40], OperandSize::Dword)
}

#[test]
fn vpmovzxbw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 48, 246], OperandSize::Qword)
}

#[test]
fn vpmovzxbw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Eight, 1810687638, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 48, 156, 242, 150, 230, 236, 107], OperandSize::Qword)
}

#[test]
fn vpmovzxbw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 48, 211], OperandSize::Dword)
}

#[test]
fn vpmovzxbw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexed(ECX, ECX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 48, 12, 201], OperandSize::Dword)
}

#[test]
fn vpmovzxbw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 48, 205], OperandSize::Qword)
}

#[test]
fn vpmovzxbw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(YMM5)), operand2: Some(IndirectDisplaced(RBX, 898608478, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 48, 171, 94, 173, 143, 53], OperandSize::Qword)
}

#[test]
fn vpmovzxbw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 48, 212], OperandSize::Dword)
}

#[test]
fn vpmovzxbw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 142, 48, 18], OperandSize::Dword)
}

#[test]
fn vpmovzxbw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 2, 125, 139, 48, 233], OperandSize::Qword)
}

#[test]
fn vpmovzxbw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(RDI, 804606440, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 141, 48, 159, 232, 81, 245, 47], OperandSize::Qword)
}

#[test]
fn vpmovzxbw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 169, 48, 244], OperandSize::Dword)
}

#[test]
fn vpmovzxbw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Eight, 540547865, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 169, 48, 132, 246, 25, 27, 56, 32], OperandSize::Dword)
}

#[test]
fn vpmovzxbw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(YMM30)), operand2: Some(Direct(XMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 2, 125, 169, 48, 246], OperandSize::Qword)
}

#[test]
fn vpmovzxbw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(YMM9)), operand2: Some(IndirectScaledIndexed(RDI, RBX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 125, 174, 48, 12, 159], OperandSize::Qword)
}

#[test]
fn vpmovzxbw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 203, 48, 216], OperandSize::Dword)
}

#[test]
fn vpmovzxbw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(ZMM6)), operand2: Some(IndirectScaledDisplaced(ECX, Four, 259836779, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 204, 48, 52, 141, 107, 203, 124, 15], OperandSize::Dword)
}

#[test]
fn vpmovzxbw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 125, 203, 48, 246], OperandSize::Qword)
}

#[test]
fn vpmovzxbw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledIndexed(RCX, RSI, Four, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 205, 48, 36, 177], OperandSize::Qword)
}

