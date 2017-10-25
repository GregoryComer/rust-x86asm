use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpsllvw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 213, 141, 18, 214], OperandSize::Dword)
}

fn vpsllvw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(ECX, 2028808087, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 229, 139, 18, 169, 151, 39, 237, 120], OperandSize::Dword)
}

fn vpsllvw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM30)), operand3: Some(Direct(XMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 146, 141, 131, 18, 245], OperandSize::Qword)
}

fn vpsllvw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVW, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM10)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Two, 1311287101, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 173, 143, 18, 140, 86, 61, 167, 40, 78], OperandSize::Qword)
}

fn vpsllvw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 171, 18, 229], OperandSize::Dword)
}

fn vpsllvw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Eight, 833505458, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 197, 172, 18, 156, 194, 178, 72, 174, 49], OperandSize::Dword)
}

fn vpsllvw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVW, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM25)), operand3: Some(Direct(YMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 50, 181, 167, 18, 246], OperandSize::Qword)
}

fn vpsllvw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVW, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM17)), operand3: Some(IndirectDisplaced(RBX, 2044000900, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 245, 165, 18, 187, 132, 250, 212, 121], OperandSize::Qword)
}

fn vpsllvw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVW, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 245, 205, 18, 202], OperandSize::Dword)
}

fn vpsllvw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVW, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Two, 5652560, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 229, 206, 18, 188, 112, 80, 64, 86, 0], OperandSize::Dword)
}

fn vpsllvw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVW, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM12)), operand3: Some(Direct(ZMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 18, 157, 202, 18, 198], OperandSize::Qword)
}

fn vpsllvw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVW, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM8)), operand3: Some(Indirect(RDI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 189, 205, 18, 55], OperandSize::Qword)
}

