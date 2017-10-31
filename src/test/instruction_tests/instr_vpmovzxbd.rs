use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovzxbd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 49, 214], OperandSize::Dword)
}

#[test]
fn vpmovzxbd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(ECX, EBX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 49, 20, 89], OperandSize::Dword)
}

#[test]
fn vpmovzxbd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 49, 248], OperandSize::Qword)
}

#[test]
fn vpmovzxbd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(RDI, RBX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 49, 12, 95], OperandSize::Qword)
}

#[test]
fn vpmovzxbd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 49, 243], OperandSize::Dword)
}

#[test]
fn vpmovzxbd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 647765828, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 49, 44, 69, 68, 31, 156, 38], OperandSize::Dword)
}

#[test]
fn vpmovzxbd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 49, 197], OperandSize::Qword)
}

#[test]
fn vpmovzxbd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledDisplaced(RAX, Four, 734440588, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 49, 4, 133, 140, 172, 198, 43], OperandSize::Qword)
}

#[test]
fn vpmovzxbd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 139, 49, 194], OperandSize::Dword)
}

#[test]
fn vpmovzxbd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(ECX, 1370720852, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 141, 49, 153, 84, 138, 179, 81], OperandSize::Dword)
}

#[test]
fn vpmovzxbd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 18, 125, 141, 49, 239], OperandSize::Qword)
}

#[test]
fn vpmovzxbd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(XMM10)), operand2: Some(IndirectDisplaced(RAX, 114116708, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 125, 137, 49, 144, 100, 72, 205, 6], OperandSize::Qword)
}

#[test]
fn vpmovzxbd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 171, 49, 223], OperandSize::Dword)
}

#[test]
fn vpmovzxbd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(YMM2)), operand2: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 170, 49, 17], OperandSize::Dword)
}

#[test]
fn vpmovzxbd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(YMM30)), operand2: Some(Direct(XMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 66, 125, 170, 49, 246], OperandSize::Qword)
}

#[test]
fn vpmovzxbd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(YMM2)), operand2: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 172, 49, 23], OperandSize::Qword)
}

#[test]
fn vpmovzxbd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 202, 49, 195], OperandSize::Dword)
}

#[test]
fn vpmovzxbd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Two, 260836524, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 203, 49, 164, 81, 172, 12, 140, 15], OperandSize::Dword)
}

#[test]
fn vpmovzxbd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(XMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 18, 125, 203, 49, 240], OperandSize::Qword)
}

#[test]
fn vpmovzxbd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(ZMM16)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 1033820121, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 125, 207, 49, 4, 141, 217, 215, 158, 61], OperandSize::Qword)
}

