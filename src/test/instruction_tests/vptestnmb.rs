use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vptestnmb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMB, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 110, 11, 38, 230], OperandSize::Dword)
}

fn vptestnmb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMB, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Eight, 303812990, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 78, 15, 38, 156, 195, 126, 209, 27, 18], OperandSize::Dword)
}

fn vptestnmb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMB, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM8)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 62, 11, 38, 231], OperandSize::Qword)
}

fn vptestnmb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMB, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 284125539, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 94, 14, 38, 20, 189, 99, 105, 239, 16], OperandSize::Qword)
}

fn vptestnmb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMB, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 94, 41, 38, 225], OperandSize::Dword)
}

fn vptestnmb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMB, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 70, 46, 38, 62], OperandSize::Dword)
}

fn vptestnmb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMB, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM17)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 118, 37, 38, 217], OperandSize::Qword)
}

fn vptestnmb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMB, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(RCX, 281110604, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 126, 45, 38, 145, 76, 104, 193, 16], OperandSize::Qword)
}

fn vptestnmb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMB, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 118, 79, 38, 236], OperandSize::Dword)
}

fn vptestnmb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMB, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 632618163, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 110, 73, 38, 36, 221, 179, 252, 180, 37], OperandSize::Dword)
}

fn vptestnmb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMB, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM12)), operand3: Some(Direct(ZMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 146, 30, 75, 38, 218], OperandSize::Qword)
}

fn vptestnmb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMB, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM17)), operand3: Some(IndirectScaledIndexed(RBX, RBX, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 118, 65, 38, 52, 219], OperandSize::Qword)
}

