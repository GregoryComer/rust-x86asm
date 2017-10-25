use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpcompressq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 138, 139, 247], OperandSize::Dword)
}

fn vpcompressq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSQ, operand1: Some(IndirectScaledDisplaced(EDX, Two, 830476968, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 253, 8, 139, 28, 85, 168, 18, 128, 49], OperandSize::Dword)
}

fn vpcompressq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSQ, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 194, 253, 140, 139, 220], OperandSize::Qword)
}

fn vpcompressq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSQ, operand1: Some(IndirectScaledDisplaced(RBX, Four, 1564933902, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 253, 8, 139, 12, 157, 14, 255, 70, 93], OperandSize::Qword)
}

fn vpcompressq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 171, 139, 222], OperandSize::Dword)
}

fn vpcompressq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSQ, operand1: Some(IndirectDisplaced(EBX, 902146980, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 253, 40, 139, 187, 164, 171, 197, 53], OperandSize::Dword)
}

fn vpcompressq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSQ, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 194, 253, 169, 139, 255], OperandSize::Qword)
}

fn vpcompressq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSQ, operand1: Some(IndirectScaledDisplaced(RDI, Four, 909305164, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM26)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 253, 40, 139, 20, 189, 76, 229, 50, 54], OperandSize::Qword)
}

fn vpcompressq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 207, 139, 201], OperandSize::Dword)
}

fn vpcompressq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSQ, operand1: Some(IndirectScaledIndexed(EDI, EDI, Two, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 253, 72, 139, 4, 127], OperandSize::Dword)
}

fn vpcompressq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 205, 139, 203], OperandSize::Qword)
}

fn vpcompressq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSQ, operand1: Some(IndirectDisplaced(RSI, 57683371, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 253, 72, 139, 142, 171, 45, 112, 3], OperandSize::Qword)
}

