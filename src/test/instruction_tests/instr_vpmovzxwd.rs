use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovzxwd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 51, 205], OperandSize::Dword)
}

#[test]
fn vpmovzxwd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(EDX, Four, 135997410, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 51, 20, 149, 226, 39, 27, 8], OperandSize::Dword)
}

#[test]
fn vpmovzxwd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 51, 216], OperandSize::Qword)
}

#[test]
fn vpmovzxwd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(RCX, Two, 248817693, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 51, 36, 77, 29, 168, 212, 14], OperandSize::Qword)
}

#[test]
fn vpmovzxwd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 51, 231], OperandSize::Dword)
}

#[test]
fn vpmovzxwd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Eight, 837716920, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 51, 172, 254, 184, 139, 238, 49], OperandSize::Dword)
}

#[test]
fn vpmovzxwd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 51, 215], OperandSize::Qword)
}

#[test]
fn vpmovzxwd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Four, 223850168, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 51, 132, 136, 184, 174, 87, 13], OperandSize::Qword)
}

#[test]
fn vpmovzxwd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 51, 247], OperandSize::Dword)
}

#[test]
fn vpmovzxwd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(ESI, 611610608, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 137, 51, 134, 240, 111, 116, 36], OperandSize::Dword)
}

#[test]
fn vpmovzxwd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 146, 125, 143, 51, 253], OperandSize::Qword)
}

#[test]
fn vpmovzxwd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(XMM27)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 110828833, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 125, 140, 51, 28, 253, 33, 29, 155, 6], OperandSize::Qword)
}

#[test]
fn vpmovzxwd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 175, 51, 223], OperandSize::Dword)
}

#[test]
fn vpmovzxwd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(YMM1)), operand2: Some(IndirectDisplaced(EDX, 806678310, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 169, 51, 138, 38, 239, 20, 48], OperandSize::Dword)
}

#[test]
fn vpmovzxwd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(YMM9)), operand2: Some(Direct(XMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 82, 125, 171, 51, 200], OperandSize::Qword)
}

#[test]
fn vpmovzxwd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(YMM22)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Two, 1145374876, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 125, 169, 51, 180, 82, 156, 8, 69, 68], OperandSize::Qword)
}

#[test]
fn vpmovzxwd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 203, 51, 201], OperandSize::Dword)
}

#[test]
fn vpmovzxwd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectDisplaced(EDX, 1638070121, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 201, 51, 162, 105, 247, 162, 97], OperandSize::Dword)
}

#[test]
fn vpmovzxwd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(YMM17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 50, 125, 207, 51, 193], OperandSize::Qword)
}

#[test]
fn vpmovzxwd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(ZMM31)), operand2: Some(IndirectDisplaced(RBX, 253848819, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 125, 201, 51, 187, 243, 108, 33, 15], OperandSize::Qword)
}

