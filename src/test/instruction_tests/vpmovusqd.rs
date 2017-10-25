use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpmovusqd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 126, 140, 21, 230], OperandSize::Dword)
}

fn vpmovusqd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQD, operand1: Some(IndirectScaledIndexed(ECX, ECX, Eight, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 21, 44, 201], OperandSize::Dword)
}

fn vpmovusqd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQD, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 146, 126, 142, 21, 198], OperandSize::Qword)
}

fn vpmovusqd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQD, operand1: Some(IndirectScaledDisplaced(RBX, Eight, 1277808753, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 8, 21, 20, 221, 113, 208, 41, 76], OperandSize::Qword)
}

fn vpmovusqd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 126, 175, 21, 249], OperandSize::Dword)
}

fn vpmovusqd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQD, operand1: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 21, 43], OperandSize::Dword)
}

fn vpmovusqd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(YMM26)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 126, 174, 21, 209], OperandSize::Qword)
}

fn vpmovusqd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQD, operand1: Some(IndirectScaledDisplaced(RAX, Eight, 638296444, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 126, 40, 21, 44, 197, 124, 161, 11, 38], OperandSize::Qword)
}

fn vpmovusqd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 126, 206, 21, 217], OperandSize::Dword)
}

fn vpmovusqd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQD, operand1: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 21, 7], OperandSize::Dword)
}

fn vpmovusqd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQD, operand1: Some(Direct(YMM20)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 178, 126, 204, 21, 220], OperandSize::Qword)
}

fn vpmovusqd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQD, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Two, 806660978, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 72, 21, 188, 88, 114, 171, 20, 48], OperandSize::Qword)
}

