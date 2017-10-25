use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpblendmb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 77, 143, 102, 245], OperandSize::Dword)
}

fn vpblendmb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(ESI, EDX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 69, 138, 102, 52, 214], OperandSize::Dword)
}

fn vpblendmb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMB, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM30)), operand3: Some(Direct(XMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 130, 13, 132, 102, 206], OperandSize::Qword)
}

fn vpblendmb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMB, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM15)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 5, 140, 102, 62], OperandSize::Qword)
}

fn vpblendmb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 171, 102, 223], OperandSize::Dword)
}

fn vpblendmb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 117, 170, 102, 39], OperandSize::Dword)
}

fn vpblendmb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMB, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM17)), operand3: Some(Direct(YMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 194, 117, 165, 102, 196], OperandSize::Qword)
}

fn vpblendmb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMB, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM19)), operand3: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 101, 167, 102, 23], OperandSize::Qword)
}

fn vpblendmb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMB, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 77, 203, 102, 220], OperandSize::Dword)
}

fn vpblendmb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMB, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Eight, 210848232, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 77, 202, 102, 172, 193, 232, 73, 145, 12], OperandSize::Dword)
}

fn vpblendmb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMB, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM8)), operand3: Some(Direct(ZMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 146, 61, 202, 102, 223], OperandSize::Qword)
}

fn vpblendmb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMB, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM8)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 1831113114, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 61, 203, 102, 12, 213, 154, 145, 36, 109], OperandSize::Qword)
}

