use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovqw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 126, 142, 52, 202], OperandSize::Dword)
}

#[test]
fn vpmovqw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQW, operand1: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Eight, 1671514103, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 52, 164, 193, 247, 71, 161, 99], OperandSize::Dword)
}

#[test]
fn vpmovqw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQW, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 178, 126, 143, 52, 203], OperandSize::Qword)
}

#[test]
fn vpmovqw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQW, operand1: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 8, 52, 30], OperandSize::Qword)
}

#[test]
fn vpmovqw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 126, 169, 52, 249], OperandSize::Dword)
}

#[test]
fn vpmovqw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQW, operand1: Some(IndirectScaledDisplaced(ESI, Eight, 1625530314, Some(OperandSize::Qword), None)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 52, 20, 245, 202, 159, 227, 96], OperandSize::Dword)
}

#[test]
fn vpmovqw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQW, operand1: Some(Direct(XMM30)), operand2: Some(Direct(YMM26)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 2, 126, 171, 52, 214], OperandSize::Qword)
}

#[test]
fn vpmovqw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQW, operand1: Some(IndirectScaledDisplaced(RBX, Four, 1152847195, Some(OperandSize::Qword), None)), operand2: Some(Direct(YMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 126, 40, 52, 4, 157, 91, 13, 183, 68], OperandSize::Qword)
}

#[test]
fn vpmovqw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 126, 204, 52, 221], OperandSize::Dword)
}

#[test]
fn vpmovqw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQW, operand1: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 52, 31], OperandSize::Dword)
}

#[test]
fn vpmovqw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQW, operand1: Some(Direct(XMM18)), operand2: Some(Direct(ZMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 50, 126, 203, 52, 210], OperandSize::Qword)
}

#[test]
fn vpmovqw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQW, operand1: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 72, 52, 39], OperandSize::Qword)
}

