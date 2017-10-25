use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpmovusdb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 126, 139, 17, 198], OperandSize::Dword)
}

fn vpmovusdb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDB, operand1: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 17, 23], OperandSize::Dword)
}

fn vpmovusdb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 126, 139, 17, 249], OperandSize::Qword)
}

fn vpmovusdb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDB, operand1: Some(IndirectScaledDisplaced(RAX, Two, 2010436724, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 126, 8, 17, 52, 69, 116, 212, 212, 119], OperandSize::Qword)
}

fn vpmovusdb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 126, 174, 17, 237], OperandSize::Dword)
}

fn vpmovusdb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDB, operand1: Some(IndirectScaledDisplaced(EAX, Four, 140634462, Some(OperandSize::Qword), None)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 17, 4, 133, 94, 233, 97, 8], OperandSize::Dword)
}

fn vpmovusdb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(YMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 126, 175, 17, 221], OperandSize::Qword)
}

fn vpmovusdb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDB, operand1: Some(IndirectDisplaced(RDX, 1206058132, Some(OperandSize::Qword), None)), operand2: Some(Direct(YMM9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 40, 17, 138, 148, 252, 226, 71], OperandSize::Qword)
}

fn vpmovusdb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 126, 203, 17, 198], OperandSize::Dword)
}

fn vpmovusdb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDB, operand1: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 17, 2], OperandSize::Dword)
}

fn vpmovusdb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(ZMM21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 126, 202, 17, 238], OperandSize::Qword)
}

fn vpmovusdb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDB, operand1: Some(IndirectScaledIndexed(RBX, RDX, Four, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM26)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 126, 72, 17, 20, 147], OperandSize::Qword)
}

