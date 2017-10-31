use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtudq2ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 127, 142, 122, 197], OperandSize::Dword)
}

#[test]
fn vcvtudq2ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(EDI, 406532148, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 127, 138, 122, 167, 52, 48, 59, 24], OperandSize::Dword)
}

#[test]
fn vcvtudq2ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 145, 127, 143, 122, 223], OperandSize::Qword)
}

#[test]
fn vcvtudq2ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PS, operand1: Some(Direct(XMM29)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Four, 192854587, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 127, 142, 122, 172, 186, 59, 186, 126, 11], OperandSize::Qword)
}

#[test]
fn vcvtudq2ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 127, 171, 122, 244], OperandSize::Dword)
}

#[test]
fn vcvtudq2ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PS, operand1: Some(Direct(YMM3)), operand2: Some(IndirectDisplaced(ECX, 1257515137, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 127, 172, 122, 153, 129, 40, 244, 74], OperandSize::Dword)
}

#[test]
fn vcvtudq2ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PS, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 193, 127, 174, 122, 229], OperandSize::Qword)
}

#[test]
fn vcvtudq2ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PS, operand1: Some(Direct(YMM17)), operand2: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 127, 175, 122, 10], OperandSize::Qword)
}

#[test]
fn vcvtudq2ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 127, 221, 122, 220], OperandSize::Dword)
}

#[test]
fn vcvtudq2ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PS, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectDisplaced(EDI, 1015138366, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 127, 202, 122, 167, 62, 200, 129, 60], OperandSize::Dword)
}

#[test]
fn vcvtudq2ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PS, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 127, 218, 122, 200], OperandSize::Qword)
}

#[test]
fn vcvtudq2ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PS, operand1: Some(Direct(ZMM19)), operand2: Some(IndirectScaledIndexed(RSI, RDX, Eight, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 127, 207, 122, 28, 214], OperandSize::Qword)
}

