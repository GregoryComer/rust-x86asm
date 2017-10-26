use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovuswb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSWB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 126, 139, 16, 249], OperandSize::Dword)
}

#[test]
fn vpmovuswb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSWB, operand1: Some(IndirectScaledDisplaced(ESI, Four, 776027610, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 16, 28, 181, 218, 61, 65, 46], OperandSize::Dword)
}

#[test]
fn vpmovuswb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSWB, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 50, 126, 137, 16, 238], OperandSize::Qword)
}

#[test]
fn vpmovuswb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSWB, operand1: Some(IndirectDisplaced(RDI, 1937726153, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 16, 191, 201, 90, 127, 115], OperandSize::Qword)
}

#[test]
fn vpmovuswb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSWB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 126, 170, 16, 233], OperandSize::Dword)
}

#[test]
fn vpmovuswb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSWB, operand1: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Two, 244270398, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 16, 148, 86, 62, 69, 143, 14], OperandSize::Dword)
}

#[test]
fn vpmovuswb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSWB, operand1: Some(Direct(XMM30)), operand2: Some(Direct(YMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 18, 126, 169, 16, 214], OperandSize::Qword)
}

#[test]
fn vpmovuswb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSWB, operand1: Some(IndirectScaledIndexed(RDX, RAX, Four, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 126, 40, 16, 28, 130], OperandSize::Qword)
}

#[test]
fn vpmovuswb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSWB, operand1: Some(Direct(YMM6)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 126, 201, 16, 206], OperandSize::Dword)
}

#[test]
fn vpmovuswb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSWB, operand1: Some(IndirectScaledIndexed(ESI, EDI, Eight, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 16, 36, 254], OperandSize::Dword)
}

#[test]
fn vpmovuswb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSWB, operand1: Some(Direct(YMM18)), operand2: Some(Direct(ZMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 50, 126, 201, 16, 210], OperandSize::Qword)
}

#[test]
fn vpmovuswb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSWB, operand1: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 16, 56], OperandSize::Qword)
}

