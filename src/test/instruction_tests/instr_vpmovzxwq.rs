use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovzxwq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 52, 237], OperandSize::Dword)
}

#[test]
fn vpmovzxwq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 52, 46], OperandSize::Dword)
}

#[test]
fn vpmovzxwq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 52, 241], OperandSize::Qword)
}

#[test]
fn vpmovzxwq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(RDX, RSI, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 52, 60, 114], OperandSize::Qword)
}

#[test]
fn vpmovzxwq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 52, 228], OperandSize::Dword)
}

#[test]
fn vpmovzxwq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexed(EDX, ECX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 52, 20, 138], OperandSize::Dword)
}

#[test]
fn vpmovzxwq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 52, 246], OperandSize::Qword)
}

#[test]
fn vpmovzxwq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(YMM4)), operand2: Some(IndirectDisplaced(RSI, 166017554, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 52, 166, 18, 58, 229, 9], OperandSize::Qword)
}

#[test]
fn vpmovzxwq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 142, 52, 251], OperandSize::Dword)
}

#[test]
fn vpmovzxwq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Two, 84538661, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 139, 52, 148, 119, 37, 245, 9, 5], OperandSize::Dword)
}

#[test]
fn vpmovzxwq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 125, 138, 52, 224], OperandSize::Qword)
}

#[test]
fn vpmovzxwq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(XMM31)), operand2: Some(IndirectScaledIndexed(RAX, RSI, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 125, 139, 52, 60, 176], OperandSize::Qword)
}

#[test]
fn vpmovzxwq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 169, 52, 253], OperandSize::Dword)
}

#[test]
fn vpmovzxwq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 1658483999, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 171, 52, 28, 125, 31, 117, 218, 98], OperandSize::Dword)
}

#[test]
fn vpmovzxwq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(YMM12)), operand2: Some(Direct(XMM29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 18, 125, 172, 52, 229], OperandSize::Qword)
}

#[test]
fn vpmovzxwq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(YMM8)), operand2: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 125, 170, 52, 6], OperandSize::Qword)
}

#[test]
fn vpmovzxwq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 202, 52, 192], OperandSize::Dword)
}

#[test]
fn vpmovzxwq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectScaledIndexed(EAX, EDX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 206, 52, 60, 208], OperandSize::Dword)
}

#[test]
fn vpmovzxwq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(XMM17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 50, 125, 204, 52, 225], OperandSize::Qword)
}

#[test]
fn vpmovzxwq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(ZMM8)), operand2: Some(IndirectDisplaced(RCX, 661186252, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 125, 201, 52, 129, 204, 230, 104, 39], OperandSize::Qword)
}

