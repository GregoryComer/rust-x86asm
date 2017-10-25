use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vptestmw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMW, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 237, 14, 38, 254], OperandSize::Dword)
}

fn vptestmw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMW, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(EAX, ECX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 237, 15, 38, 52, 200], OperandSize::Dword)
}

fn vptestmw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMW, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM21)), operand3: Some(Direct(XMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 178, 213, 5, 38, 240], OperandSize::Qword)
}

fn vptestmw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMW, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM8)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 189, 13, 38, 63], OperandSize::Qword)
}

fn vptestmw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMW, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 221, 45, 38, 248], OperandSize::Dword)
}

fn vptestmw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMW, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 1987815753, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 237, 41, 38, 28, 133, 73, 169, 123, 118], OperandSize::Dword)
}

fn vptestmw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMW, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM26)), operand3: Some(Direct(YMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 146, 173, 34, 38, 237], OperandSize::Qword)
}

fn vptestmw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMW, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM17)), operand3: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 245, 33, 38, 35], OperandSize::Qword)
}

fn vptestmw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMW, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 229, 73, 38, 238], OperandSize::Dword)
}

fn vptestmw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMW, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM0)), operand3: Some(Indirect(EAX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 74, 38, 56], OperandSize::Dword)
}

fn vptestmw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMW, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 205, 74, 38, 242], OperandSize::Qword)
}

fn vptestmw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMW, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Four, 1804424523, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 213, 73, 38, 140, 139, 75, 85, 141, 107], OperandSize::Qword)
}

