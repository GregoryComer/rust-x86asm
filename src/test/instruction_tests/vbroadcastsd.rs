use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vbroadcastsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSD, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 536340398, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 25, 44, 181, 174, 231, 247, 31], OperandSize::Dword)
}

fn vbroadcastsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSD, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexed(RAX, RAX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 25, 20, 64], OperandSize::Qword)
}

fn vbroadcastsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 25, 241], OperandSize::Dword)
}

fn vbroadcastsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 25, 193], OperandSize::Qword)
}

fn vbroadcastsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 170, 25, 200], OperandSize::Dword)
}

fn vbroadcastsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSD, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledDisplaced(ECX, Four, 152618784, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 169, 25, 44, 141, 32, 199, 24, 9], OperandSize::Dword)
}

fn vbroadcastsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSD, operand1: Some(Direct(YMM23)), operand2: Some(Direct(XMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 130, 253, 172, 25, 248], OperandSize::Qword)
}

fn vbroadcastsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSD, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledIndexed(RBX, RDI, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 175, 25, 28, 251], OperandSize::Qword)
}

fn vbroadcastsd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 204, 25, 217], OperandSize::Dword)
}

fn vbroadcastsd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSD, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Eight, 1459812992, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 207, 25, 148, 198, 128, 250, 2, 87], OperandSize::Dword)
}

fn vbroadcastsd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSD, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 253, 203, 25, 239], OperandSize::Qword)
}

fn vbroadcastsd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSD, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectDisplaced(RBX, 1581938705, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 203, 25, 163, 17, 120, 74, 94], OperandSize::Qword)
}

