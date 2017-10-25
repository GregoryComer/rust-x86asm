use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpmovswb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSWB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 126, 142, 32, 211], OperandSize::Dword)
}

fn vpmovswb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSWB, operand1: Some(IndirectDisplaced(ECX, 1387862606, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 32, 161, 78, 26, 185, 82], OperandSize::Dword)
}

fn vpmovswb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSWB, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 146, 126, 143, 32, 245], OperandSize::Qword)
}

fn vpmovswb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSWB, operand1: Some(IndirectDisplaced(RDX, 373479107, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 32, 178, 195, 214, 66, 22], OperandSize::Qword)
}

fn vpmovswb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSWB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 126, 175, 32, 211], OperandSize::Dword)
}

fn vpmovswb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSWB, operand1: Some(IndirectDisplaced(ESI, 1967877790, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 32, 142, 158, 110, 75, 117], OperandSize::Dword)
}

fn vpmovswb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSWB, operand1: Some(Direct(XMM12)), operand2: Some(Direct(YMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 82, 126, 173, 32, 244], OperandSize::Qword)
}

fn vpmovswb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSWB, operand1: Some(IndirectScaledIndexed(RDX, RBX, Two, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 126, 40, 32, 36, 90], OperandSize::Qword)
}

fn vpmovswb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSWB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 126, 202, 32, 217], OperandSize::Dword)
}

fn vpmovswb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSWB, operand1: Some(IndirectScaledIndexedDisplaced(EAX, ECX, Eight, 1862832824, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 32, 156, 200, 184, 146, 8, 111], OperandSize::Dword)
}

fn vpmovswb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSWB, operand1: Some(Direct(YMM28)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 146, 126, 201, 32, 220], OperandSize::Qword)
}

fn vpmovswb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSWB, operand1: Some(IndirectDisplaced(RCX, 10190606, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 126, 72, 32, 137, 14, 127, 155, 0], OperandSize::Qword)
}

