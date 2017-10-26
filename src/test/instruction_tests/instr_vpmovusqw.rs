use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovusqw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 126, 143, 20, 218], OperandSize::Dword)
}

#[test]
fn vpmovusqw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQW, operand1: Some(IndirectDisplaced(EDI, 1070002296, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 20, 183, 120, 240, 198, 63], OperandSize::Dword)
}

#[test]
fn vpmovusqw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQW, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 146, 126, 137, 20, 238], OperandSize::Qword)
}

#[test]
fn vpmovusqw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQW, operand1: Some(IndirectDisplaced(RDI, 2053405351, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 126, 8, 20, 135, 167, 122, 100, 122], OperandSize::Qword)
}

#[test]
fn vpmovusqw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 126, 173, 20, 236], OperandSize::Dword)
}

#[test]
fn vpmovusqw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQW, operand1: Some(IndirectScaledDisplaced(EBX, Four, 2068577445, Some(OperandSize::Qword), None)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 20, 4, 157, 165, 252, 75, 123], OperandSize::Dword)
}

#[test]
fn vpmovusqw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQW, operand1: Some(Direct(XMM23)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 178, 126, 170, 20, 247], OperandSize::Qword)
}

#[test]
fn vpmovusqw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQW, operand1: Some(IndirectDisplaced(RAX, 1181244719, Some(OperandSize::Qword), None)), operand2: Some(Direct(YMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 126, 40, 20, 128, 47, 93, 104, 70], OperandSize::Qword)
}

#[test]
fn vpmovusqw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 126, 207, 20, 249], OperandSize::Dword)
}

#[test]
fn vpmovusqw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQW, operand1: Some(IndirectDisplaced(EBX, 1784732045, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 20, 131, 141, 217, 96, 106], OperandSize::Dword)
}

#[test]
fn vpmovusqw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQW, operand1: Some(Direct(XMM28)), operand2: Some(Direct(ZMM31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 2, 126, 203, 20, 252], OperandSize::Qword)
}

#[test]
fn vpmovusqw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQW, operand1: Some(IndirectScaledDisplaced(RDX, Four, 88900910, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 126, 72, 20, 4, 149, 46, 133, 76, 5], OperandSize::Qword)
}

