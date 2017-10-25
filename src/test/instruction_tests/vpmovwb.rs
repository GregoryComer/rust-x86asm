use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpmovwb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVWB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 126, 139, 48, 240], OperandSize::Dword)
}

fn vpmovwb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVWB, operand1: Some(IndirectScaledDisplaced(ECX, Four, 1736281361, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 48, 44, 141, 17, 141, 125, 103], OperandSize::Dword)
}

fn vpmovwb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVWB, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 130, 126, 139, 48, 228], OperandSize::Qword)
}

fn vpmovwb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVWB, operand1: Some(IndirectDisplaced(RCX, 294336218, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 126, 8, 48, 177, 218, 54, 139, 17], OperandSize::Qword)
}

fn vpmovwb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVWB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 126, 175, 48, 219], OperandSize::Dword)
}

fn vpmovwb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVWB, operand1: Some(IndirectScaledDisplaced(EDI, Two, 1011629282, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 48, 36, 125, 226, 60, 76, 60], OperandSize::Dword)
}

fn vpmovwb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVWB, operand1: Some(Direct(XMM29)), operand2: Some(Direct(YMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 18, 126, 174, 48, 229], OperandSize::Qword)
}

fn vpmovwb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVWB, operand1: Some(IndirectScaledIndexed(RBX, RSI, Two, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 126, 40, 48, 4, 115], OperandSize::Qword)
}

fn vpmovwb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVWB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 126, 204, 48, 229], OperandSize::Dword)
}

fn vpmovwb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVWB, operand1: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Eight, 295672572, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 48, 132, 254, 252, 154, 159, 17], OperandSize::Dword)
}

fn vpmovwb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVWB, operand1: Some(Direct(YMM30)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 146, 126, 207, 48, 222], OperandSize::Qword)
}

fn vpmovwb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVWB, operand1: Some(IndirectScaledDisplaced(RCX, Two, 237569752, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 72, 48, 36, 77, 216, 6, 41, 14], OperandSize::Qword)
}

