use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovzxwq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 52, 214], OperandSize::Dword)
}

#[test]
fn vpmovzxwq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(EDI, 964128422, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 52, 175, 166, 110, 119, 57], OperandSize::Dword)
}

#[test]
fn vpmovzxwq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 52, 221], OperandSize::Qword)
}

#[test]
fn vpmovzxwq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 52, 57], OperandSize::Qword)
}

#[test]
fn vpmovzxwq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 52, 248], OperandSize::Dword)
}

#[test]
fn vpmovzxwq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(YMM3)), operand2: Some(IndirectDisplaced(EDX, 516277025, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 52, 154, 33, 195, 197, 30], OperandSize::Dword)
}

#[test]
fn vpmovzxwq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 52, 255], OperandSize::Qword)
}

#[test]
fn vpmovzxwq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Eight, 1343417122, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 52, 180, 219, 34, 235, 18, 80], OperandSize::Qword)
}

#[test]
fn vpmovzxwq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 141, 52, 209], OperandSize::Dword)
}

#[test]
fn vpmovzxwq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 139, 52, 48], OperandSize::Dword)
}

#[test]
fn vpmovzxwq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 130, 125, 137, 52, 196], OperandSize::Qword)
}

#[test]
fn vpmovzxwq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Eight, 1740479651, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 139, 52, 180, 194, 163, 156, 189, 103], OperandSize::Qword)
}

#[test]
fn vpmovzxwq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 172, 52, 207], OperandSize::Dword)
}

#[test]
fn vpmovzxwq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(YMM0)), operand2: Some(IndirectDisplaced(EAX, 1594425080, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 174, 52, 128, 248, 254, 8, 95], OperandSize::Dword)
}

#[test]
fn vpmovzxwq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(XMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 146, 125, 172, 52, 230], OperandSize::Qword)
}

#[test]
fn vpmovzxwq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(YMM13)), operand2: Some(IndirectScaledIndexed(RBX, RAX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 125, 174, 52, 44, 195], OperandSize::Qword)
}

#[test]
fn vpmovzxwq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 202, 52, 218], OperandSize::Dword)
}

#[test]
fn vpmovzxwq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(ZMM6)), operand2: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 202, 52, 51], OperandSize::Dword)
}

#[test]
fn vpmovzxwq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(XMM25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 130, 125, 202, 52, 201], OperandSize::Qword)
}

#[test]
fn vpmovzxwq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWQ, operand1: Some(Direct(ZMM26)), operand2: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 125, 202, 52, 23], OperandSize::Qword)
}

