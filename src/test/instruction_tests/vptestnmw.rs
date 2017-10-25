use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vptestnmw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMW, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 238, 10, 38, 209], OperandSize::Dword)
}

fn vptestnmw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMW, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Two, 1398454096, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 246, 10, 38, 172, 79, 80, 183, 90, 83], OperandSize::Dword)
}

fn vptestnmw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMW, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 238, 10, 38, 221], OperandSize::Qword)
}

fn vptestnmw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMW, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM13)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 150, 14, 38, 38], OperandSize::Qword)
}

fn vptestnmw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMW, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 214, 44, 38, 247], OperandSize::Dword)
}

fn vptestnmw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMW, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 238, 47, 38, 27], OperandSize::Dword)
}

fn vptestnmw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMW, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 210, 222, 41, 38, 239], OperandSize::Qword)
}

fn vptestnmw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMW, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM25)), operand3: Some(IndirectDisplaced(RDI, 1539487484, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 182, 35, 38, 159, 252, 182, 194, 91], OperandSize::Qword)
}

fn vptestnmw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMW, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 230, 78, 38, 252], OperandSize::Dword)
}

fn vptestnmw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMW, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectDisplaced(EDX, 722626344, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 230, 74, 38, 154, 40, 103, 18, 43], OperandSize::Dword)
}

fn vptestnmw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMW, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 210, 238, 75, 38, 219], OperandSize::Qword)
}

fn vptestnmw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMW, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectDisplaced(RSI, 986019130, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 206, 75, 38, 166, 58, 117, 197, 58], OperandSize::Qword)
}

